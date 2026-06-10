//! # Macros
//!
//! `cirious_codex_result` provides several macros for easily creating `CodexOk` and `CodexError` instances.

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

/// Macro for immediate exit from a function with a `CodexError`.
///
/// Use this macro to propagate an error when a failure condition is met.
/// It automatically injects the caller's location and accepts optional
/// metadata for extended diagnostics.
///
/// # Examples
///
/// ```
/// use cirious_codex_result::{codex_bail, codex_ok, Result};
///
/// fn find_user(id: u32) -> Result<String> {
///     if id == 0 {
///         codex_bail!("INVALID_ID", "User ID cannot be zero", "attempted" => "0");
///     }
///     codex_ok!("User".to_string().into())
/// }
/// ```
#[macro_export]
macro_rules! codex_bail {
    ($name:expr, $cause:expr $(, $key:expr => $val:expr)* $(,)?) => {
        return Err(
            $crate::CodexError::builder($name, $cause)
                $(.with_meta($key, $val))*
        );
    };
}

/// Macro for validating a condition and returning a `CodexError` if it fails.
///
/// Acts as an assertion that, when false, exits the function with an error
/// document containing the provided context and metadata.
///
/// # Examples
///
/// ```
/// use cirious_codex_result::{codex_ensure, codex_ok, Result};
///
/// fn login(token: &str) -> Result<()> {
///     codex_ensure!(!token.is_empty(), "AUTH_FAILED", "Token is missing");
///     codex_ok!(())
/// }
/// ```
#[macro_export]
macro_rules! codex_ensure {
    ($cond:expr, $name:expr, $cause:expr $(, $key:expr => $val:expr)* $(,)?) => {
        if !($cond) {
            return Err(
                $crate::CodexError::builder($name, $cause)
                    $(.with_meta($key, $val))*
            );
        }
    };
}
