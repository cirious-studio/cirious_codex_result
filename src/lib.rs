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

// Enables docs.rs features to show tags like "Only on Windows"
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Ensures all public items are documented (Essential for crates.io)
#![warn(missing_docs)]
// Prevents accidental unsafe code in the entire crate
#![deny(unsafe_code)]

pub mod error;
pub mod macros;
pub mod ok;

pub use error::{CodexError, IntoCodex};
pub use ok::{CodexOk, CodexOkRaw, CodexOkWrap, ExecutionContext};

#[cfg(feature = "serde")]
pub use ok::log_codex_ok;

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

#[cfg(test)]
mod tests {
  use crate::codex_ok;

  use super::{codex_bail, codex_ensure, Result};

  fn test_bail_function(val: i32) -> Result<i32> {
    if val < 0 {
      codex_bail!("VAL_ERROR", "Value is negative", "val" => val.to_string());
    }
    codex_ok!(val)
  }

  fn test_ensure_function(val: i32) -> Result<i32> {
    codex_ensure!(val >= 0, "VAL_ERROR", "Value must be non-negative", "val" => val.to_string());
    codex_ok!(val)
  }

  #[test]
  fn test_codex_bail() {
    let result = test_bail_function(-1);
    assert!(result.is_err());

    match result {
      Err(err) => {
        assert_eq!(err.name(), "VAL_ERROR");
        assert_eq!(err.cause(), "Value is negative");
        if let Some(meta) = err.metadata().get("val") {
          assert_eq!(meta, "-1");
        }
      }
      Ok(_) => panic!("Expected an error"),
    }
  }

  #[test]
  fn test_codex_ensure_success() {
    let result = test_ensure_function(10);
    assert!(result.is_ok());
    match result {
      Ok(val) => assert_eq!(val.value, 10),
      Err(err) => panic!("Unexpected error: {err}"),
    }
  }

  #[test]
  fn test_codex_ensure_failure() {
    let result = test_ensure_function(-5);
    assert!(result.is_err());

    match result {
      Err(err) => {
        assert_eq!(err.name(), "VAL_ERROR");
        assert_eq!(err.cause(), "Value must be non-negative");
        if let Some(meta) = err.metadata().get("val") {
          assert_eq!(meta, "-5");
        }
      }
      Ok(_) => panic!("Expected an error"),
    }
  }
}
