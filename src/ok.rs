//! Success result encapsulation for the Cirious Codex diagnostic framework.
//!
//! This module provides the [`CodexOk`] type, which serves as the foundational
//! success wrapper. It captures execution context (like the caller location) and
//! allows arbitrary metadata to be attached to successful operations for advanced
//! observability and tracing.

use std::collections::HashMap;
use std::fmt;
use std::panic::Location;

/// A convenient alias for [`CodexOkRaw`] using the default dynamic metadata container.
///
/// This type simplifies usage when strongly-typed contexts are not required,
/// allowing for quick key-value pair diagnostic injection.
pub type CodexOk<T> = CodexOkRaw<T, HashMap<String, String>>;

/// Detailed diagnostic information for a successful execution.
///
/// `CodexOkRaw` wraps the underlying success value and automatically captures
/// the precise file and line number where the success was instantiated.
/// It also provides a metadata envelope for injecting contextual data
/// (e.g., execution time, row counts, or process IDs) into the success path.
///
/// # Type Parameters
///
/// * `T` - The type of the successful value being wrapped.
/// * `C` - The type of the execution metadata. Defaults to `HashMap<String, String>`
///   for dynamic key-value pairs, but can be replaced with a strongly-typed struct
///   for better observability.
///
/// # Fields
///
/// * `value` - The underlying success outcome.
/// * `location` - The source code location (file and line) of the `CodexOkRaw` instantiation.
///   This field is ignored during serialization.
/// * `execution_meta` - A container for diagnostic information associated with the
///   successful execution.
///
/// # Examples
///
/// ```
/// use cirious_codex_result::CodexOkRaw;
/// use std::collections::HashMap;
///
/// // Explicitly provide the type to guide the compiler's inference
/// let result: CodexOkRaw<&str, HashMap<String, String>> = CodexOkRaw::new("Operation complete")
///     .with_meta("duration_ms", "42")
///     .with_meta("affected_rows", "5");
///
/// assert_eq!(result.value, "Operation complete");
/// assert_eq!(result.execution_meta.get("duration_ms").unwrap(), "42");
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CodexOkRaw<T, C = HashMap<String, String>> {
  /// The underlying successful value.
  pub value: T,
  /// The precise location in the source code where this success was created.
  #[cfg_attr(feature = "serde", serde(skip))]
  pub location: &'static Location<'static>,
  /// Arbitrary key-value metadata associated with this successful execution.
  pub execution_meta: C,
}

impl<T, C> CodexOkRaw<T, C> {
  /// Wraps the result in a success scenario, natively capturing the location.
  ///
  /// This method uses the `#[track_caller]` attribute to ensure that the
  /// location captured is the site where `CodexOkRaw::new` was called, rather
  /// than the location inside the `new` function itself.
  ///
  /// # Arguments
  ///
  /// * `value` - The successful outcome value to be wrapped.
  #[track_caller]
  pub fn new(value: T) -> Self
  where
    C: Default,
  {
    Self {
      value,
      location: Location::caller(),
      execution_meta: C::default(),
    }
  }
}

// Only implement the string-based builder if the context is a HashMap
impl<T, S: ::std::hash::BuildHasher> CodexOkRaw<T, HashMap<String, String, S>> {
  /// Injects arbitrary execution metadata into the success envelope.
  ///
  /// This method uses a builder-like pattern to allow chainable insertions
  /// of diagnostic metadata key-value pairs.
  ///
  /// # Arguments
  ///
  /// * `key` - The metadata key (e.g., `"duration_ms"`).
  /// * `value` - The string representation of the metadata value.
  #[must_use]
  pub fn with_meta<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
    self.execution_meta.insert(key.into(), value.into());
    self
  }
}

impl<T: fmt::Display, C> fmt::Display for CodexOkRaw<T, C> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "SUCCESS: {} | Location: {}:{}",
      self.value,
      self.location.file(),
      self.location.line()
    )
  }
}

/// Represents the structured diagnostic context for a successful operation.
///
/// This struct is intended to be used as the generic metadata context `C` within
/// a [`CodexOkRaw`] instance. It provides standard fields for
/// tracking common performance and identity metrics across your application.
///
/// # Fields
///
/// * `duration_ms` - The time taken for the operation to complete, in milliseconds.
/// * `affected_rows` - The number of database rows, file records, or entities modified.
/// * `process_id` - The unique identifier of the process or worker thread that performed the task.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct ExecutionContext {
  /// The time taken for the operation to complete, in milliseconds.
  pub duration_ms: u64,
  /// The number of database rows, file records, or entities modified.
  pub affected_rows: usize,
  /// The unique identifier of the process or worker thread that performed the task.
  pub process_id: u32,
}

impl ExecutionContext {
  /// Creates a new `ExecutionContext` with default values.
  ///
  /// This is the canonical way to create a default context when the
  /// `default` method is not directly accessible or for explicit clarity.
  #[inline]
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the duration of the operation.
  #[inline]
  #[must_use]
  pub const fn with_duration(mut self, duration_ms: u64) -> Self {
    self.duration_ms = duration_ms;
    self
  }

  /// Sets the number of affected rows.
  #[inline]
  #[must_use]
  pub const fn with_affected_rows(mut self, affected_rows: usize) -> Self {
    self.affected_rows = affected_rows;
    self
  }

  /// Sets the process ID.
  #[inline]
  #[must_use]
  pub const fn with_process_id(mut self, process_id: u32) -> Self {
    self.process_id = process_id;
    self
  }
}

/// Serializes a value to JSON and prints it to standard output.
///
/// This function is intended for structured logging where the provided value
/// must implement `serde::Serialize`. If serialization fails, the error is
/// silently ignored.
///
/// # Arguments
///
/// * `val` - A reference to the value to be serialized.
///
/// # Example
///
/// ```
/// # use serde::Serialize;
/// # use cirious_codex_result::CodexOkWrap;
///
/// # #[derive(Serialize)]
/// # struct User { name: String }
/// # let user = User { name: "Alice".into() };
/// cirious_codex_result::log_codex_ok(&user.into_codex());
/// ```
#[cfg(feature = "serde")]
pub fn log_codex_ok<T, C>(result: &CodexOkRaw<T, C>)
where
  T: serde::Serialize,
  C: serde::Serialize,
{
  if let Ok(json) = serde_json::to_string(result) {
    println!("{json}");
  }
}

/// Extension trait to ergonomically convert any value into a `CodexOkRaw`.
pub trait CodexOkWrap: Sized {
  /// Wraps the current value into a `CodexOkRaw`, natively capturing the caller location.
  #[track_caller]
  fn into_codex(self) -> CodexOkRaw<Self>;
}

impl<T> CodexOkWrap for T {
  #[track_caller]
  fn into_codex(self) -> CodexOkRaw<Self> {
    CodexOkRaw::new(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_codex_ok_creation() {
    // Explicitly annotate the type to satisfy compiler inference for the HashMap
    let ok_result = CodexOk::new(42).with_meta("duration_ms", "10");

    assert_eq!(ok_result.value, 42);
    assert!(ok_result.location.file().ends_with("ok.rs"));

    let res = ok_result.execution_meta.get("duration_ms");
    assert_eq!(res.map(std::string::String::as_str), Some("10"));
  }
}
