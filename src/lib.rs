// src/lib.rs
pub mod error;
pub mod ok;

// Re-export so users of the library can call it directly:
// cirious_codex_result::CodexError instead of cirious_codex_result::error::CodexError
pub use error::CodexError;
pub use ok::CodexOk;

/// The core diagnostic result type for the Cirious ecosystem.
pub type Result<T, E = CodexError> = std::result::Result<CodexOk<T>, E>;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_codex_result_alias() {
    // Test function that returns our global Result
    fn simulate_operation(success: bool) -> Result<i32> {
      if success {
        Ok(CodexOk::new(100).with_meta("duration_ms", "15"))
      } else {
        Err(
          CodexError::builder("OP_ERROR", "Simulation failed")
            .with_suggestion("Check the success boolean parameter")
            .with_meta("retry_count", "3"),
        )
      }
    }

    // Validating success scenario
    let success_res = simulate_operation(true);
    assert!(success_res.is_ok());
    assert_eq!(success_res.unwrap().value, 100);

    // Validating failure scenario
    let error_res = simulate_operation(false);
    assert!(error_res.is_err());
    assert_eq!(error_res.unwrap_err().name, "OP_ERROR");
  }
}
