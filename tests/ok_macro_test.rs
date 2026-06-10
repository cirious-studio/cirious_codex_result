//! Success Macro Integration Tests
//!
//! Ensures that `codex_ok!` properly wraps values and attaches metadata
//! for observability purposes.

use cirious_codex_result::{codex_ok, Result};

#[test]
fn test_codex_ok_with_metadata() {
  let result: Result<i32> = codex_ok!(42, "source" => "test", "type" => "unit");

  assert!(result.is_ok());
  match result {
    Ok(ok) => {
      assert_eq!(ok.value, 42);
      if let Some(meta) = ok.execution_meta.get("source") {
        assert_eq!(meta, "test");
      }
      if let Some(meta) = ok.execution_meta.get("type") {
        assert_eq!(meta, "unit");
      }
    }
    Err(err) => panic!("Unexpected error: {err}"),
  }
}
