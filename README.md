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

---

## ✨ Features

- Core diagnostic result types (`CodexOk` and `CodexError`).
- Automatic caller location tracking via `#[track_caller]`.
- Native backtrace capturing for deep diagnostics.
- Ergonomic Builder pattern for context and suggestion injection.
- Extension Traits (`.into_codex()`) and ergonomic macros (`codex_ok!`) for frictionless success wrapping.


---

## 🚀 Quick Start
 
Add the following to your `Cargo.toml`:

```toml
[dependencies]
cirious_codex_result = "0.1.0"
```

And then in your code:

```rust
use cirious_codex_result::{codex_ok, CodexError, Result};
 
fn connect_to_database(url: &str) -> Result<String> {
    if url.is_empty() {
        return Err(
            CodexError::builder("DB_ERROR", "URL cannot be empty")
                .with_suggestion("Provide a valid connection string like postgres://...")
                .with_meta("attempted_url", url),
        );
    }

    // Wrap the successful value with contextual metadata
    codex_ok!("Connected".to_string(), "latency_ms" => "25")
}

fn main() {
    match connect_to_database("postgres://localhost") {
        Ok(ok) => {
            println!("✅ Success: {}", ok.value);
            println!("⏱️ Latency: {}ms", ok.execution_meta.get("latency_ms").unwrap());
        }
        Err(err) => {
            println!("❌ Error [{}]: {}", err.name, err.cause);
            if let Some(sug) = err.suggestion {
                println!("💡 Suggestion: {}", sug);
            }
            // You can also access err.backtrace and err.metadata!
        }
    }
}
```
---

## 🚧 Current Status & Roadmap

The architecture is currently being mapped out for the initial `v0.2` release. Planned features include:

- [ ] **Macros**: `codex_bail!` & `codex_ensure!` for quick propagation.
- [ ] **Conversions**: `From` traits for standard errors (e.g., `std::io::Error`).
- [ ] **Serde**: Optional feature for serialization & deserialization.
- [ ] **Formatting**: Advanced formatters for CLI outputs & structured logs.
- [ ] **Metadata**: Typed values & generic contexts replacing `HashMap`.

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
