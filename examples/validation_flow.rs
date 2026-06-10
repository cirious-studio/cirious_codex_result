//! Validation Flow Example
//!
//! This example demonstrates how to use the `codex_ensure!` macro to perform
//! business logic validations, significantly reducing boilerplate code while
//! maintaining high-context error reporting.

use cirious_codex_result::{codex_ensure, codex_ok, Result};

/// Processes a customer order with validation checks.
///
/// This function validates that the order identifier is positive and that the
/// order amount is greater than zero.
///
/// # Arguments
/// * `id` - The unique identifier for the order.
/// * `amount` - The monetary value of the order.
///
/// # Returns
/// A `Result` containing the success message on success, or a `CodexError`
/// with diagnostic metadata if validation fails.
fn process_order(id: u32, amount: f64) -> Result<String> {
  // Business logic validation using assertions that propagate errors
  codex_ensure!(id > 0, "INVALID_ID", "Order ID must be positive", "id" => id.to_string());
  codex_ensure!(amount > 0.0, "INVALID_AMOUNT", "Amount must be > 0", "amount" => amount.to_string());

  codex_ok!(format!("Order {} processed", id), "status" => "complete")
}

/// Entry point demonstrating the validation flow.
fn main() {
  match process_order(101, 50.5) {
    Ok(res) => println!("✅ {}", res.value),
    Err(e) => println!("❌ Error: {}", e.cause()),
  }
}
