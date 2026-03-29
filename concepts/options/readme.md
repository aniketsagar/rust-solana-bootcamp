🧠 Concept 1: Option<T> + Pattern Matching

This is one of the most important primitives in Rust.

If you master this early, your Rust becomes immediately more idiomatic.

Rust replaces null with:

```rust
Option<T>
```

which is:

```rust
enum Option<T> {
    Some(T),
    None,
}

```

Meaning:

value exists → Some(value)
value missing → None

No crashes. No surprises. Compiler forces correctness.
