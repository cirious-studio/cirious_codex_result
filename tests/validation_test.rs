//! Validation Integration Tests
//!
//! Verifies the behavior of the `codex_ensure!` macro under various conditions
//! to ensure proper error propagation and metadata handling.

use cirious_codex_result::{codex_ensure, codex_ok, Result};

/// A function to simulate order validation logic.
fn validate_order(id: u32, amount: f64) -> Result<()> {
  codex_ensure!(id > 0, "INVALID_ID", "ID must be > 0", "id" => id.to_string());
  codex_ensure!(amount > 0.0, "INVALID_AMOUNT", "Amount must be > 0", "amount" => amount.to_string());
  codex_ok!(())
}

#[test]
fn test_validation_success() {
  let result = validate_order(1, 10.0);
  assert!(result.is_ok());
}

#[test]
fn test_validation_failure_id() {
  let result = validate_order(0, 10.0);
  assert!(result.is_err());

  match result {
    Err(err) => {
      assert_eq!(err.name(), "INVALID_ID");
      if let Some(meta) = err.metadata().get("id") {
        assert_eq!(meta, "0");
      }
    }
    Ok(_) => panic!("Expected an error"),
  }
}
