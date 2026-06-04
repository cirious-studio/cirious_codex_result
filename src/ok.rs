// src/ok.rs
use std::collections::HashMap;
use std::panic::Location;

/// Detailed diagnostic information for a successful execution.
#[derive(Debug)]
pub struct CodexOk<T> {
  pub value: T,
  pub location: &'static Location<'static>,
  pub execution_meta: HashMap<String, String>,
}

impl<T> CodexOk<T> {
  /// Wraps the result in a success scenario, natively capturing the location.
  #[track_caller]
  pub fn new(value: T) -> Self {
    Self {
      value,
      location: Location::caller(),
      execution_meta: HashMap::new(),
    }
  }

  /// Injects arbitrary execution metadata into the success envelope.
  pub fn with_meta<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
    self.execution_meta.insert(key.into(), value.into());
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_codex_ok_creation() {
    let ok_result = CodexOk::new(42)
      .with_meta("duration_ms", "10");

    assert_eq!(ok_result.value, 42);
    assert!(ok_result.location.file().ends_with("ok.rs"));
    assert_eq!(ok_result.execution_meta.get("duration_ms").unwrap(), "10");
  }
}
