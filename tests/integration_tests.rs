#![allow(missing_docs)]

use cirious_codex_result::{codex_ok, CodexError, CodexOkWrap, Result};

// Testamos um fluxo completo simulando uma operação externa
fn connect_to_database(url: &str) -> Result<String> {
  if url.is_empty() {
    return Err(
      CodexError::builder("DB_ERROR", "URL cannot be empty")
        .with_suggestion("Provide a valid connection string like postgres://...")
        .with_meta("attempted_url", url),
    );
  }

  codex_ok!("Connected".to_string(), "latency_ms" => "25")
}

#[test]
fn test_successful_connection() {
  let result = connect_to_database("postgres://localhost");
  assert!(result.is_ok());

  let ok = result.unwrap();
  assert_eq!(ok.value, "Connected");
  assert_eq!(ok.execution_meta.get("latency_ms").unwrap(), "25");
  assert!(ok.location.file().ends_with("integration_tests.rs"));
}

#[test]
fn test_failed_connection() {
  let result = connect_to_database("");
  assert!(result.is_err());

  let err = result.unwrap_err();
  assert_eq!(err.name(), "DB_ERROR");
  assert_eq!(
    err.suggestion().unwrap(),
    "Provide a valid connection string like postgres://..."
  );
  assert!(err.location().file().ends_with("integration_tests.rs"));
}

// ==========================================
// ERGONOMICS TESTS (Macro and Trait)
// ==========================================

#[test]
fn test_codex_ok_macro_simple() {
  // Test using the macro without metadata
  fn simple_return() -> Result<i32> {
    codex_ok!(99)
  }

  let result = simple_return().unwrap();
  assert_eq!(result.value, 99);
  assert!(result.execution_meta.is_empty());
}

#[test]
fn test_codex_ok_macro_with_meta() {
  // Test using the macro with multiple chained metadata
  fn meta_return() -> Result<&'static str> {
    codex_ok!("Success", "step" => "1", "user_id" => "A123")
  }

  let result = meta_return().unwrap();
  assert_eq!(result.value, "Success");
  assert_eq!(result.execution_meta.get("step").unwrap(), "1");
  assert_eq!(result.execution_meta.get("user_id").unwrap(), "A123");
}

#[test]
fn test_codex_ok_extension_trait() {
  // Test using the .into_codex() method injected into all types
  fn trait_return() -> Result<f64> {
    Ok(42.5.into_codex().with_meta("precision", "high"))
  }

  let result = trait_return().unwrap();
  assert_eq!(result.value, 42.5);
  assert_eq!(result.execution_meta.get("precision").unwrap(), "high");
}
