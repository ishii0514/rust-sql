---
name: sql-parser-tdd
description: Extend or modify the Rust SQL parser in this repository. Use when Codex needs to add SQL syntax, adjust the Pest grammar, update AST nodes, change expression parsing, or follow the repository's TDD workflow while editing src/sql.pest, src/parser.rs, src/expression.rs, src/ast.rs, README.md, or related tests.
---

# SQL Parser TDD

## Overview

Follow the repository's TDD workflow when changing SQL parsing behavior.
Keep edits aligned across grammar, AST, parser, expression handling, tests, and docs.

## Use This Workflow

1. Identify the requested SQL behavior and map it to the affected files before editing.
2. Add or update tests first in the closest `#[cfg(test)]` module.
3. Run the narrowest failing test that proves the missing behavior.
4. Implement the smallest parser or AST change needed to make that test pass.
5. Re-run the focused tests, then the full test suite.
6. Finish with formatting and lint checks when code changed materially.

## Map Changes To Files

- Edit `src/sql.pest` when the accepted SQL syntax changes.
- Edit `src/ast.rs` when the parsed structure needs a new node, field, or enum variant.
- Edit `src/parser.rs` when statement-level parsing changes for `SELECT`, `INSERT`, `UPDATE`, or `DELETE`.
- Edit `src/expression.rs` when operator precedence, literals, or standalone expression parsing changes.
- Edit `README.md` when supported SQL features or examples change.

## Keep These Invariants

- Preserve SQL keyword case-insensitivity.
- Preserve support for comments and whitespace handling.
- Keep Japanese text support in string literals intact.
- Prefer extending existing enums and structs instead of introducing parallel representations.
- Keep public APIs documented with `///` comments when adding new public items.

## Write Tests First

- Add parser tests near the code they validate instead of creating separate test files.
- Cover a happy path and at least one edge case for each syntax addition.
- Include regression coverage for precedence-sensitive expression changes.
- Add failure tests when invalid syntax should stay rejected.

## Verify Changes

Run these commands in order as needed:

```bash
cargo test
cargo fmt
cargo clippy
```

When iterating on one feature, prefer a focused `cargo test <name>` run before the full suite.

## Check Before Finishing

- Confirm grammar rules and Rust match arms stay in sync.
- Confirm AST changes are reflected in parser assertions.
- Confirm README examples still match supported syntax.
- Summarize which tests were added first and which commands were run.
