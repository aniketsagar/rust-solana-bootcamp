
# Concept 1 — Option<T> Deep Chapter (Part 1)
## Why Rust Invented Option<T>

In many languages (C, C++, Java, Python, JS), absence of value is represented using:

- null
- None
- nullptr
- undefined

These are NOT type-safe.

Example problem in other languages:

value.length()

If value == null → crash at runtime.

Rust prevents this class of bug entirely.

Instead:

enum Option<T> {
    Some(T),
    None
}

This means:

absence is encoded in the type system

Compiler forces you to handle it.

--------------------------------

Key insight:

Option<T> is NOT just a container.

It is a control-flow encoding mechanism.
