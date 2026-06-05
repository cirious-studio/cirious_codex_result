#![allow(missing_docs)]

use cirious_codex_result::{codex_ok, CodexError, Result};

fn process_payment(amount: f64) -> Result<()> {
  if amount <= 0.0 {
    return Err(
      CodexError::builder("INVALID_AMOUNT", "Payment amount must be greater than zero")
        .with_suggestion("Check the cart subtotal before processing payment")
        .with_meta("amount_passed", amount.to_string()),
    );
  }

  // Success flow utilizing the new ergonomic macro
  codex_ok!((), "transaction_id" => "TRX-998123")
}

fn main() {
  println!("--- Simulating success ---");
  match process_payment(150.50) {
    Ok(ok) => {
      println!("✅ Success!");
      println!("Transaction ID: {}", ok.execution_meta.get("transaction_id").unwrap());
    }
    Err(e) => println!("❌ Failure: {}", e),
  }

  println!("\n--- Simulating error ---");
  match process_payment(-5.0) {
    Ok(_) => println!("✅ Success!"),
    Err(e) => {
      println!("❌ Captured failure:\n{}", e);
      println!("Metadata: {:?}", e.metadata());
      // You can even print the backtrace here if enabled:
      // println!("Backtrace:\n{}", e.backtrace);
    }
  }
}
