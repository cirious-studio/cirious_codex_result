//! # Cirious Codex Result
//!
//! `cirious_codex_result` is a robust diagnostic framework tailored for the Cirious
//! ecosystem. It provides an enhanced replacement for the standard library's `Result`
//! type, enforcing rich context, caller location tracking, and structured metadata
//! on both success (`Ok`) and failure (`Err`) paths.
//!
//! ## Overview
//!
//! - [`CodexError`]: Represents a failed execution with actionable suggestions, location tracking, and backtraces.
//! - [`CodexOk`]: Wraps successful executions, allowing the injection of metrics or metadata.
//! - [`Result`]: The central alias uniting `CodexOk` and `CodexError`.

pub mod error;
pub mod ok;

pub use error::CodexError;
pub use ok::{CodexOk, CodexOkWrap};

/// The core diagnostic result type for the Cirious ecosystem.
///
/// This type alias sets `CodexOk<T>` as the default success type and `CodexError`
/// as the default error type. By using this alias instead of `std::result::Result`,
/// every operation is automatically primed to carry diagnostic execution metadata.
///
/// # Examples
///
/// ```
/// use cirious_codex_result::{Result, CodexOk, CodexError};
///
/// fn process_data(valid: bool) -> Result<i32> {
///     if valid {
///         Ok(CodexOk::new(200).with_meta("status", "success"))
///     } else {
///         Err(CodexError::builder("INVALID_DATA", "The provided data was invalid"))
///     }
/// }
///
/// assert!(process_data(true).is_ok());
/// assert!(process_data(false).is_err());
/// ```
pub type Result<T, E = CodexError> = std::result::Result<CodexOk<T>, E>;

/// Macro for quickly wrapping a value and optional metadata into an `Ok(CodexOk)`.
///
/// # Examples
///
/// ```
/// use cirious_codex_result::{codex_ok, Result};
///
/// fn process() -> Result<&'static str> {
///     // Returns immediately with metadata
///     codex_ok!("Done", "time_ms" => "12", "id" => "99")
/// }
/// ```
#[macro_export]
macro_rules! codex_ok {
  // Chamada simples sem metadados
  ($val:expr) => {
    Ok($crate::CodexOk::new($val))
  };
  // Chamada com injeção de N metadados separados por vírgula
  ($val:expr, $($key:expr => $meta:expr),+ $(,)?) => {
    Ok($crate::CodexOk::new($val)
      $(.with_meta($key, $meta))+)
  };
}

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
    assert_eq!(error_res.unwrap_err().name(), "OP_ERROR");
  }
}
