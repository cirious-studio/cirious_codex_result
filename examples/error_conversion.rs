//! Error Conversion Example
//!
//! Demonstrates how to transform standard library errors into rich `CodexError`
//! diagnostics using the `IntoCodex` trait.

use cirious_codex_result::{codex_ok, IntoCodex, Result};
use std::fs;

/// Attempts to read a configuration file, converting IO errors to `CodexErrors`.
///
/// # Returns
/// A `Result` containing the file content or a diagnostic `CodexError`.
fn load_config(path: &str) -> Result<String> {
  let content = fs::read_to_string(path).map_err(|e| e.into_codex("CONFIG_READ_FAILURE"))?;

  codex_ok!(content, "path" => path.to_string())
}

/// Entry point demonstrating the error conversion flow.
fn main() {
  if let Err(e) = load_config("non_existent.toml") {
    println!("❌ Error Name: {}", e.name());
    println!("   Cause: {}", e.cause());
  }
}
