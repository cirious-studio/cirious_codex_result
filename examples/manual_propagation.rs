//! Manual Propagation Example
//!
//! This example demonstrates how to use the `codex_bail!` macro to interrupt execution
//! and propagate detailed diagnostic information when a failure condition occurs.

use cirious_codex_result::{codex_bail, codex_ok, Result};

/// Simulates fetching a remote configuration with a timeout constraint.
///
/// # Arguments
/// * `timeout_ms` - The configured timeout in milliseconds.
///
/// # Returns
/// A `Result` containing the configuration string on success, or a `CodexError`
/// if the timeout value is considered unsafe.
fn fetch_remote_config(timeout_ms: u64) -> Result<String> {
  if timeout_ms < 100 {
    codex_bail!(
        "TIMEOUT_EXCEEDED",
        "Config fetch took too long",
        "timeout" => timeout_ms.to_string(),
        "system" => "remote_api"
    );
  }
  codex_ok!("config_data".to_string())
}

/// Entry point demonstrating error handling with manual propagation.
fn main() {
  if let Err(e) = fetch_remote_config(50) {
    // Accessing diagnostic fields directly
    println!("Error Name: {}", e.name());
    println!("Metadata: {:?}", e.metadata());
  }
}
