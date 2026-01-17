# Agent Guidelines

## Philosophy

- Keep code minimal; do not add features, flags, or abstractions unless asked.
- Avoid defensive code and "just in case" checks.
- Let errors bubble up unless there is a clear reason to handle them.
- Avoid hidden side effects. Make mutations explicit.
- Treat tests as servants of the design: if tests block a clearer intended structure, rewrite the tests to validate the new shape instead of bending code to preserve old test seams.

## Coding Style

- Tests live in separate modules (`foo/test.rs`), never inline `#[cfg(test)]`.
- Parent modules with children must use `foo/mod.rs` with `foo/*.rs`; do not use `foo.rs` alongside a `foo/` directory.
- Prefer ordering definitions so structs, functions, and helpers appear before they are referenced.
- Define types/errors/helpers before public functions that use them.
- Avoid tiny helper functions that do very little, especially when they are only used once; prefer inlining unless reuse or a real readability win justifies the helper.
- Prefer free functions over private `impl` helpers for internal pipeline steps that only need a subset of an owner's fields. Pass exactly the fields/data the step needs so dependencies and borrows stay constrained. Use private methods when the operation semantically belongs to the receiver or maintains its invariants.
- Prefer named constants over inline magic numbers.
- Avoid no-op assignments, including redundant aliasing.

### Aesthetics

- Prefer explicit `return` where possible.
- No single-line `if` bodies; always use braces on the next line.

### Naming

- Names do not need to repeat context already provided by their module. Use short role names inside focused modules, and add extra context to the item name only when the name would otherwise be ambiguous in ordinary use.
- If the verb is only `get`, omit it when the receiver and object name already make the lookup clear.
- Use `new` for lightweight, stateless construction, such as creating a vector or data value. Use a verb like `create`, `build`, or `load` when construction does work, such as computation, allocation, IO, GPU setup, or other side effects.

### Visibility

- Only use `pub` and private visibility.
- Default to private; make items `pub` only when they must cross a module boundary.
- If a child-module item is needed by the parent, make the item and its parent-called methods `pub` while keeping the child module private.
- Prevent cross-module mutation of internal state unless the mutation happens through an explicit API, or the struct is a transient data carrier owned by the caller.

## Validation

- Run `cargo fmt`.
- Run `cargo clippy`; avoid unexpected warnings.
- Do a lightweight pass to check for AGENTS.md infractions.
