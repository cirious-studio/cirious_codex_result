//! Chaining Example
//!
//! This example demonstrates how to use the `?` operator with `cirious_codex_result`
//! to compose multiple fallible operations into a clean, sequential flow.

use cirious_codex_result::{codex_ok, Result};

/// Performs the initial step in the sequence.
///
/// # Returns
/// A `Result` containing the initial integer value wrapped in `CodexOk`.
fn step_one() -> Result<i32> {
  codex_ok!(10)
}

/// Performs a transformation step on the provided value.
///
/// # Arguments
/// * `val` - The input integer to be processed.
///
/// # Returns
/// A `Result` containing the doubled integer value wrapped in `CodexOk`.
fn step_two(val: i32) -> Result<i32> {
  codex_ok!(val * 2)
}

/// Demonstrates sequential error handling using the `?` operator.
///
/// This function showcases how the `CodexResult` framework seamlessly integrates
/// with Rust's standard error propagation mechanism.
///
/// # Returns
/// `Ok(())` if the chain completes, or an error if any step fails.
fn chaining_example() -> Result<()> {
  // The `?` operator extracts the value from `CodexOk` and propagates errors
  let val = step_one()?.value;
  let result = step_two(val)?;

  println!("Final value: {}", result.value);
  codex_ok!(())
}

fn main() {
  if let Err(e) = chaining_example() {
    println!("❌ Error: {}", e.cause());
  }
}
