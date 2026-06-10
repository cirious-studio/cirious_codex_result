//! Propagation Integration Tests
//!
//! Validates that `codex_bail!` forces an immediate return with the
//! expected error details.

use cirious_codex_result::{codex_bail, codex_ok, Result};

/// Simulates a system failure triggered by a specific condition.
fn perform_action(trigger: bool) -> Result<String> {
  if trigger {
    codex_bail!("ACTION_FAILED", "System failure triggered", "code" => "500");
  }
  codex_ok!("success".to_string())
}

#[test]
fn test_codex_bail_propagation() {
  let result = perform_action(true);
  assert!(result.is_err());

  match result {
    Err(err) => {
      assert_eq!(err.name(), "ACTION_FAILED");
      if let Some(meta) = err.metadata().get("code") {
        assert_eq!(meta, "500");
      }
    }
    Ok(_) => panic!("Expected an error"),
  }
}
