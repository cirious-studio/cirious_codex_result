<div align="center">

# 🛡️ Cirious Codex Result

**Robust Result & Error Handling Framework**

[![CI](https://github.com/cirious-studio/cirious_codex_result/actions/workflows/ci.yml/badge.svg)](https://github.com/cirious-studio/cirious_codex_result/actions/workflows/ci.yml) [![Crates.io](https://img.shields.io/crates/v/cirious_codex_result.svg)](https://crates.io/crates/cirious_codex_result) [![Docs.rs](https://docs.rs/cirious_codex_result/badge.svg)](https://docs.rs/cirious_codex_result) [![Language](https://img.shields.io/badge/Language-Rust-black?logo=rust)](https://www.rust-lang.org/) [![License](https://img.shields.io/badge/License-MIT%2FApache-blue.svg)](#-license)

</div>

---

## 📖 Overview

**Cirious Codex Result** is a highly optimized, dependency-free foundational library designed not just for error handling, but as a complete **Diagnostic & Tracking Framework**. 

It provides a rich, generic envelope around operations, guaranteeing that every execution—whether successful (`Ok`) or failed (`Err`)—generates a detailed diagnostic document containing precise caller locations (file/line), contextual metadata, resolution suggestions, and full execution backtraces.

Designed to be the immutable bedrock for execution tracking within the Cirious ecosystem, prioritizing maximum observability and flawless developer experience.

## 🚀 Quick Start
 
Add the following to your `Cargo.toml`:

```toml
[dependencies]
cirious_codex_result = "0.2.0"
```

And then in your code:

```rust
use cirious_codex_result::{codex_bail, codex_ensure, codex_ok, Result};

fn connect_to_database(url: &str) -> Result<String> {
  // Using codex_ensure! to validate inputs with high-context diagnostics
  codex_ensure!(!url.is_empty(), "DB_ERROR", "URL cannot be empty", "attempted_url" => url);

  if url == "fail" {
    // Using codex_bail! to propagate errors with custom metadata
    codex_bail!("CONNECTION_FAILED", "Database unreachable", "system" => "postgres");
  }

  // Wrapping success with contextual metadata
  codex_ok!("Connected".to_string(), "latency_ms" => "25")
}

fn main() {
  match connect_to_database("postgres://localhost") {
    Ok(ok) => println!("✅ Success: {}", ok.value),
    Err(err) => println!("❌ Error [{}]: {}", err.name(), err.cause()),
  }
}
```
---

## 🚧 Current Status & Roadmap

### ✅ v0.1.0 — Foundation

- [x] Core diagnostic result types (`CodexOkRaw` and `CodexError`).
- [x] Automatic caller location tracking via `#[track_caller]`.
- [x] Native backtrace capturing for deep diagnostics.
- [x] Ergonomic Builder pattern for context and suggestion injection.
- [x] Extension Traits (`.into_codex()`) and ergonomic macros (`codex_ok!`) for frictionless success wrapping.

### ✅ v0.2.0 — Production Refinement

- [x] Macros `codex_bail!` & `codex_ensure!` for quick propagation.
- [x] Conversions `From` traits for standard errors (e.g., `std::io::Error`).
- [x] Serde Optional feature for serialization & deserialization.
- [x] Formatting Advanced formatters for CLI outputs & structured logs.
- [x] Metadata Typed values & generic contexts replacing `HashMap`.
- [x] Core diagnostic result types (CodexOkRaw and CodexError).

### 🔭 v0.3.0 — Advanced Diagnostics

- [ ] Native ? Operator Support: Implement From<E> for CodexError traits to allow seamless error propagation using the standard ? operator.
- [ ] Zero-Cost Diagnostics: Optimize Backtrace capturing; introduce an #[cfg(feature = "full-debug")] flag to allow users to toggle heavy diagnostic data in production environments.
- [ ] Unified Result Macro: Introduce codex_result! to handle both success and error paths dynamically, reducing boilerplate in controller/service layers.
- [ ] Async Readiness: Ensure CodexOk and CodexError are Send + Sync and fully compatible with tokio and other async runtimes.
- [ ] Type-Safe Diagnostics: Explore const generics or specialized traits to enforce mandatory metadata for specific error types (e.g., forcing a service_id for all Database errors).

---

## 📜 License

Licensed under either of the following, at your option:

* **[MIT License](LICENSE-MIT)**
* **[Apache License 2.0](LICENSE-APACHE)**

---

<div align="center">
  <i>Minimalist by design. Consistent in execution.</i><br>
  <sub>Engineered by Cirious Studio</sub>
</div>
