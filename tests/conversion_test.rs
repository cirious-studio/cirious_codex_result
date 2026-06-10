//! Conversion Integration Tests
//!
//! Validates the seamless transformation of standard library errors into
//! the `CodexError` diagnostic structure.

use cirious_codex_result::{CodexError, IntoCodex};
use std::io;

#[test]
fn test_io_error_conversion() {
  // Simulate a standard IO error
  let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found on disk");

  // Transform into CodexError
  let codex_err: CodexError = io_err.into_codex("FILE_NOT_FOUND");

  // Validate diagnostics
  assert_eq!(codex_err.name(), "FILE_NOT_FOUND");
  assert!(codex_err.cause().contains("File not found on disk"));
}

#[test]
fn test_generic_error_conversion() {
  // The idiomatic "let...else" pattern for test failure handling
  let Err(error) = "Generic failure".parse::<i32>() else {
    panic!("Expected parsing to fail, but it succeeded");
  };

  let codex_err = error.into_codex("PARSE_FAILURE");

  assert_eq!(codex_err.name(), "PARSE_FAILURE");
}
