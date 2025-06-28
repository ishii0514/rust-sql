# CLAUDE.md



This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based SQL parser implementation using the Pest parsing library. The project implements a basic SQL parser that supports SQL92 compliant syntax with an Abstract Syntax Tree (AST) representation.

## Conversation Guidelines

- 常に日本語で会話する

## Development Philosophy

### Test-Driven Development (TDD)

- 原則としてt-wadaの推奨する手法に基づいたテスト駆動開発（TDD）で進める
- 期待される入出力に基づき、まずテストを作成する
- 実装コードは書かず、テストのみを用意する
- テストを実行し、失敗を確認する
- テストが正しいことを確認できた段階でコミットする
- その後、テストをパスさせる実装を進める
- 実装中はテストを変更せず、コードを修正し続ける
- すべてのテストが通過するまで繰り返す

## Development Commands

### Building and Testing
- Build: `cargo build`
- Run tests: `cargo test`
- Run with optimizations: `cargo build --release`
- Check code: `cargo check`
- Run clippy lints: `cargo clippy`
- Format code: `cargo fmt`

### Running the Parser
The project includes a main binary that can be executed with `cargo run`.

## Code Conventions (from .cursor/rules/)

### Rust Best Practices
- Use `rustfmt` for formatting before commits
- Run `cargo clippy` regularly and fix warnings
- Follow Rust naming conventions:
  - Variables/functions: `snake_case`
  - Types: `PascalCase`
  - Constants: `UPPER_SNAKE_CASE`
- Error handling: Use `Result<T, E>` for recoverable errors, `panic!` for unrecoverable
- Document all public items with `///` comments
- Write unit tests for all public functions
- Place tests in `#[cfg(test)]` modules

### SOLID Principles
- Single Responsibility: Each struct/module should have one responsibility
- Use traits for extensibility and abstraction
- Prefer small, focused traits over large ones
- Depend on traits rather than concrete implementations

## Parser Implementation Details

The parser supports:
- Case-insensitive SQL keywords
- SQL92 compliant
- Comments (starting with `--`)
- Whitespace handling
- Japanese text in string literals

The AST is designed as a simple enum structure that can be extended for more complex SQL features like WHERE clauses, JOINs, and expressions.

## Testing Strategy

The codebase includes comprehensive unit tests covering:
- Valid SQL statement parsing
- Error cases for invalid SQL
- Case sensitivity handling
- Whitespace and comment handling
- Unicode (Japanese) text support
- Edge cases (empty lists, special characters)

