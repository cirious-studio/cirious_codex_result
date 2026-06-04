// src/error.rs
use std::backtrace::Backtrace;
use std::collections::HashMap;
use std::panic::Location;

/// Detailed diagnostic information for a failed execution.
#[derive(Debug)]
pub struct CodexError {
  pub name: String,
  pub cause: String,
  pub suggestion: Option<String>,
  pub location: &'static Location<'static>,
  pub backtrace: Backtrace,
  pub metadata: HashMap<String, String>,
}

impl CodexError {
  /// Initializes the construction of a detailed error.
  /// Automatically captures the caller's file/line location and the full backtrace.
  #[track_caller]
  pub fn builder<N: Into<String>, C: Into<String>>(name: N, cause: C) -> Self {
    Self {
      name: name.into(),
      cause: cause.into(),
      suggestion: None,
      location: Location::caller(),
      backtrace: Backtrace::capture(),
      metadata: HashMap::new(),
    }
  }

  /// Injects a resolution suggestion into the error context.
  pub fn with_suggestion<S: Into<String>>(mut self, suggestion: S) -> Self {
    self.suggestion = Some(suggestion.into());
    self
  }

  /// Injects arbitrary metadata into the error context.
  pub fn with_meta<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
    self.metadata.insert(key.into(), value.into());
    self
  }
}

impl std::fmt::Display for CodexError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}] {}", self.name, self.cause)?;
    if let Some(sug) = &self.suggestion {
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

    let loc = error.location;
    assert!(loc.file().ends_with("error.rs"), "Should capture the current file name");
    assert_eq!(error.metadata.get("process_id").unwrap(), "1234");
  }
}
