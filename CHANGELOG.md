# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project follows [Semantic Versioning](https://semver.org/).

## [0.2.0] - 2026-06-10

### Added
- **Propagation Macros**: Introduced `codex_bail!` for immediate exit with diagnostics and `codex_ensure!` for assertive validation.
- **Conversion Traits**: Implemented `From` traits for standard errors (e.g., `std::io::Error`) for seamless integration.
- **Serde Support**: Added optional `serde` feature flag for serialization/deserialization of results and metadata.
- **Improved Type System**: Refactored to use `CodexOk` as an ergonomic alias for dynamic metadata, with `CodexOkRaw` for power-user generic control.
- **Flexible Contexts**: Support for both dynamic `HashMap`-based metadata and strongly-typed `ExecutionContext` structures.

### Changed
- **Nomenclature**: Renamed the base struct to `CodexOkRaw` to allow `CodexOk` to function as the primary user-facing alias.
- **Documentation**: Updated README and examples to reflect the cleaner, type-safe API.

### Fixed
- **Type Inference**: Resolved `E0282` compiler errors in examples and doctests by standardizing type aliases and explicit annotations.

---

## [0.1.0] - 2026-05-15

### Added
- **Foundation**: Core diagnostic types `CodexOkRaw` and `CodexError`.
- **Location Tracking**: Automatic caller location (file/line) tracking via `#[track_caller]`.
- **Diagnostics**: Native backtrace capturing.
- **Fluent API**: Builder pattern for context and suggestion injection.
- **Macros**: Initial `codex_ok!` macro for frictionless success wrapping.
