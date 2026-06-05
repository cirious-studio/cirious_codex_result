//! Error result encapsulation for the Cirious Codex diagnostic framework.
//!
//! This module provides the [`CodexError`] type, which represents a robust
//! diagnostic payload for failed operations. It captures file and line locations,
//! full backtraces, and supports structured metadata and actionable suggestions.

use std::backtrace::Backtrace;
use std::collections::HashMap;
use std::panic::Location;

#[derive(Debug)]
struct CodexErrorData {
  pub name: String,
  pub cause: String,
  pub suggestion: Option<String>,
  pub location: &'static Location<'static>,
  pub backtrace: Backtrace,
  pub metadata: HashMap<String, String>,
}

/// Detailed diagnostic information for a failed execution.
///
/// `CodexError` goes beyond a standard error by acting as a diagnostic document.
/// It automatically captures the exact location where it was built and the
/// system backtrace. It also allows injecting hints/suggestions for resolution
/// and context metadata.
///
/// # Examples
///
/// ```
/// use cirious_codex_result::CodexError;
///
/// let error = CodexError::builder("DB_TIMEOUT", "Connection to database timed out")
///     .with_suggestion("Check the network connection and database status")
///     .with_meta("timeout_ms", "5000");
///
/// assert_eq!(error.name(), "DB_TIMEOUT");
/// assert_eq!(error.suggestion().unwrap(), "Check the network connection and database status");
/// ```
#[derive(Debug)]
pub struct CodexError {
  inner: Box<CodexErrorData>,
}

impl CodexError {
  /// Initializes the construction of a detailed error.
  ///
  /// This method automatically captures the caller's file/line location and the full
  /// backtrace. Using `#[track_caller]` ensures the location is traced to the invocation
  /// of `builder`, not the implementation details inside.
  ///
  /// # Arguments
  ///
  /// * `name` - A short string identifying the error category or code.
  /// * `cause` - A string describing the underlying reason for the failure.
  #[track_caller]
  pub fn builder<N: Into<String>, C: Into<String>>(name: N, cause: C) -> Self {
    Self {
      inner: Box::new(CodexErrorData {
        name: name.into(),
        cause: cause.into(),
        suggestion: None,
        location: Location::caller(),
        backtrace: Backtrace::capture(),
        metadata: HashMap::new(),
      }),
    }
  }

  /// Injects a resolution suggestion into the error context.
  ///
  /// This allows the developer to provide a hint to the end-user or the logging
  /// system on how to potentially fix the issue.
  ///
  /// # Arguments
  ///
  /// * `suggestion` - The actionable hint text.
  pub fn with_suggestion<S: Into<String>>(mut self, suggestion: S) -> Self {
    self.inner.suggestion = Some(suggestion.into());
    self
  }

  /// Injects arbitrary metadata into the error context.
  ///
  /// Useful for attaching request IDs, query parameters, or state information
  /// that helps in diagnosing the failure later.
  ///
  /// # Arguments
  ///
  /// * `key` - The metadata key (e.g., `"process_id"`).
  /// * `value` - The string representation of the metadata value.
  pub fn with_meta<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
    self.inner.metadata.insert(key.into(), value.into());
    self
  }

  /// Retrieves the high-level identifier or code for the error.
  pub fn name(&self) -> &str {
    &self.inner.name
  }

  /// Retrieves the descriptive reason or cause of the error.
  pub fn cause(&self) -> &str {
    &self.inner.cause
  }

  /// Retrieves the optional actionable hint for resolving the error.
  pub fn suggestion(&self) -> Option<&String> {
    self.inner.suggestion.as_ref()
  }

  /// Retrieves the arbitrary key-value metadata providing execution context.
  pub fn metadata(&self) -> &HashMap<String, String> {
    &self.inner.metadata
  }

  /// Retrieves the system backtrace captured at the moment of error creation.
  pub fn backtrace(&self) -> &Backtrace {
    &self.inner.backtrace
  }

  /// Retrieves the exact location in the source code where this error was built.
  pub fn location(&self) -> &'static Location<'static> {
    self.inner.location
  }
}

impl std::fmt::Display for CodexError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}] {}", self.inner.name, self.inner.cause)?;
    if let Some(sug) = &self.inner.suggestion {
      write!(f, "\nSuggestion: {}", sug)?;
    }
    Ok(())
  }
}

impl std::error::Error for CodexError {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_codex_error_display_and_creation() {
    let error = CodexError::builder("SYS_FAULT", "Insufficient memory")
      .with_suggestion("Increase the swap limit")
      .with_meta("process_id", "1234");

    let error_string = format!("{}", error);
    assert!(error_string.contains("[SYS_FAULT] Insufficient memory"));
    assert!(error_string.contains("Suggestion: Increase the swap limit"));

    let loc = error.location();
    assert!(loc.file().ends_with("error.rs"), "Should capture the current file name");
    assert_eq!(error.metadata().get("process_id").unwrap(), "1234");
  }
}
