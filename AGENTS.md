# Repository Guidelines

**重要：ユーザーには必ず日本語で返答してください。**

## Project Structure & Module Organization
- `src/lib.rs` exposes the library API; `src/main.rs` provides a small CLI entry point.
- Parser logic lives in `src/parser.rs` and `src/sql.pest` (Pest grammar).
- AST types are in `src/ast.rs`, with expression parsing in `src/expression.rs`.
- Tests are colocated in `#[cfg(test)]` modules within source files.
- Build artifacts are generated in `target/`.

## Build, Test, and Development Commands
- `cargo build`: compile the library and binary.
- `cargo run`: run the parser CLI.
- `cargo test`: run unit tests.
- `cargo fmt`: format code with rustfmt.
- `cargo clippy`: run lint checks.
- `cargo build --release`: optimized build.

## Coding Style & Naming Conventions
- Use `rustfmt` before commits; keep default Rust 4-space indentation.
- Naming follows Rust conventions: `snake_case` for functions/vars, `PascalCase` for types, `UPPER_SNAKE_CASE` for constants.
- Public items should have `///` doc comments.
- Prefer `Result<T, E>` for recoverable errors and `panic!` for unrecoverable cases.

## Testing Guidelines
- Tests use Rust’s built-in test framework; keep tests close to code in `#[cfg(test)]` modules.
- Add tests for all public functions and parser edge cases (e.g., whitespace, comments, case-insensitivity).
- No explicit coverage threshold is enforced.

## Commit & Pull Request Guidelines
- Commit messages follow Conventional Commits seen in history (e.g., `feat: ...`, `docs: ...`, `test: ...`, `chore: ...`).
- Issue-based workflow: create a feature branch per issue, e.g., `feature/issue-123-short-desc`.
- PRs should include a short summary, implementation checklist, test status, and linked issue (see `CLAUDE.md` for the template).

## Development Workflow Notes
- This repo follows a TDD flow: write tests first, confirm failure, implement, then refactor.
- Run `cargo fmt` and `cargo clippy` before opening a PR.
