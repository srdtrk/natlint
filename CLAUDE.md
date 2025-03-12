# CLAUDE.md - Natlint Project Guide

## Build Commands
- Build: `just build` or `cargo build --bin natlint --release --locked`
- Test all: `just test` or `cargo test --all --locked -- --nocapture`
- Test single: `just test "test_name"` or `cargo test test_name --locked -- --nocapture`
- Format: `cargo fmt --all`
- Lint: `cargo clippy`
- Run: `cargo run -- [args]` or `just run -- [args]`

## Code Style Guidelines
- **Rust Edition**: 2021
- **Documentation**: All public items must have doc comments (`//!` for modules, `///` for items)
- **Error Handling**: Use `thiserror` for error types and `eyre` for application errors
- **Naming**: Follow Rust conventions (snake_case for variables/functions, PascalCase for types)
- **Imports**: Group by external crates then internal modules
- **Code Organization**: Modular structure with rules organized by item type
- **Strictness**: `#![deny(clippy::nursery, clippy::pedantic, warnings, missing_docs)]`
- **Macros**: Use defined macros (`no_comment_rule`, `missing_comment_rule`, etc.) for rule patterns
- **Architecture**: Trait-based design with `Rule<T>` trait for defining rules
- **Dependencies**: Use defined versions with minimal features

## Project Overview
This project is a Solidity natspec linter with clear module separation between parser, rules, and CLI components. Rules are organized by Solidity item types (contract, struct, function, etc.) and use a plugin architecture for extensibility.