//! Examples of how to use the Cirious Codex diagnostic framework.
//!
//! This module demonstrates the usage of `CodexOk` with both dynamic metadata
//! (`HashMap`) and strongly-typed diagnostic contexts.

use cirious_codex_result::{CodexOk, CodexOkRaw, ExecutionContext};

#[cfg(feature = "serde")]
use cirious_codex_result::log_codex_ok;

/// Demonstrates dynamic metadata usage via `HashMap`.
pub fn example_dynamic_metadata() {
  println!("--- Dynamic Metadata Example ---");

  // Explicitly annotate type to satisfy inference for HashMap
  let result = CodexOk::new("Operation completed").with_meta("service", "auth");

  println!("{result}");

  #[cfg(feature = "serde")]
  log_codex_ok(&result);
}

/// Demonstrates strongly-typed context for high-performance, structured diagnostics.
pub fn example_typed_metadata() {
  println!("\n--- Typed Metadata Example ---");

  let ctx = ExecutionContext::new()
    .with_duration(150)
    .with_affected_rows(12)
    .with_process_id(9982);

  // Provide explicit type here so the compiler knows C is ExecutionContext
  let result: CodexOkRaw<String, ExecutionContext> = CodexOkRaw {
    value: "Batch processing completed".to_string(),
    location: std::panic::Location::caller(),
    execution_meta: ctx,
  };

  println!("{result}");

  #[cfg(feature = "serde")]
  log_codex_ok(&result);
}

fn main() {
  example_dynamic_metadata();
  example_typed_metadata();
}
