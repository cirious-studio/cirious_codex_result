//! Unit tests for the Cirious Codex diagnostic framework.
//!
//! Verifies source location capture, builder ergonomics, and envelope serialization.

#[cfg(test)]
mod tests {
  use cirious_codex_result::{CodexOk, CodexOkRaw, CodexOkWrap, ExecutionContext};

  #[test]
  fn test_codex_ok_creation_dynamic() {
    let result = CodexOk::new(42).with_meta("status", "ok");

    assert_eq!(result.value, 42);
    assert!(result.location.file().contains("integration_test.rs"));
  }

  #[test]
  fn test_codex_ok_creation_typed() {
    let ctx = ExecutionContext::new().with_duration(100);
    let result: CodexOkRaw<i32, ExecutionContext> = CodexOkRaw {
      value: 100,
      location: std::panic::Location::caller(),
      execution_meta: ctx,
    };

    assert_eq!(result.value, 100);
    assert_eq!(result.execution_meta.duration_ms, 100);
  }

  #[test]
  fn test_into_codex_extension() {
    let val = "extension_test";
    let result = val.into_codex();

    assert_eq!(result.value, "extension_test");
  }
}
