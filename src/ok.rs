//! Success result encapsulation for the Cirious Codex diagnostic framework.
//!
//! This module provides the [`CodexOk`] type, which serves as the foundational
//! success wrapper. It captures execution context (like the caller location) and
//! allows arbitrary metadata to be attached to successful operations for advanced
//! observability and tracing.

use std::collections::HashMap;
use std::fmt;
use std::panic::Location;

/// Detailed diagnostic information for a successful execution.
///
/// `CodexOk` wraps the underlying success value and automatically captures
/// the precise file and line number where the success was instantiated.
/// It also provides a metadata envelope for injecting contextual data
/// (e.g., execution time, row counts, or process IDs) into the success path.
///
/// # Examples
///
/// ```
/// use cirious_codex_result::CodexOk;
///
/// let result = CodexOk::new("Operation complete")
///     .with_meta("duration_ms", "42")
///     .with_meta("affected_rows", "5");
///
/// assert_eq!(result.value, "Operation complete");
/// assert_eq!(result.execution_meta.get("duration_ms").unwrap(), "42");
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CodexOk<T> {
  /// The underlying successful value.
  pub value: T,
  /// The precise location in the source code where this success was created.
  #[cfg_attr(feature = "serde", serde(skip))]
  pub location: &'static Location<'static>,
  /// Arbitrary key-value metadata associated with this successful execution.
  pub execution_meta: HashMap<String, String>,
}

impl<T: fmt::Display> fmt::Display for CodexOk<T> {
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

impl<T> CodexOk<T> {
  /// Wraps the result in a success scenario, natively capturing the location.
  ///
  /// This method uses the `#[track_caller]` attribute to ensure that the
  /// location captured is the site where `CodexOk::new` was called, rather
  /// than the location inside the `new` function itself.
  ///
  /// # Arguments
  ///
  /// * `value` - The successful outcome value to be wrapped.
  #[track_caller]
  pub fn new(value: T) -> Self {
    Self {
      value,
      location: Location::caller(),
      execution_meta: HashMap::new(),
    }
  }

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
/// # #[derive(Serialize)]
/// # struct User { name: String }
/// # let user = User { name: "Alice".into() };
/// cirious_codex_result::log_as_json(&user);
/// ```
#[cfg(feature = "serde")]
pub fn log_as_json<T: serde::Serialize>(val: &T) {
  if let Ok(json) = serde_json::to_string(val) {
    println!("{}", json);
  }
}

/// Extension trait to ergonomically convert any value into a `CodexOk`.
pub trait CodexOkWrap: Sized {
  /// Wraps the current value into a `CodexOk`, natively capturing the caller location.
  #[track_caller]
  fn into_codex(self) -> CodexOk<Self>;
}

impl<T> CodexOkWrap for T {
  #[track_caller]
  fn into_codex(self) -> CodexOk<Self> {
    CodexOk::new(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_codex_ok_creation() {
    let ok_result = CodexOk::new(42).with_meta("duration_ms", "10");

    assert_eq!(ok_result.value, 42);
    assert!(ok_result.location.file().ends_with("ok.rs"));

    let res = ok_result.execution_meta.get("duration_ms");
    if let Some(res) = res {
      assert_eq!(res, "10");
    }
  }
}
