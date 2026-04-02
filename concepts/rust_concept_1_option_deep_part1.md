
# Rust Mastery — Concept 1 (Deep) Part 1
## Option<T> as Control-Flow Algebra

---
# Why Option<T> Exists

Rust replaces null with:

enum Option<T> {
    Some(T),
    None,
}

Reasons:

1. Prevent null pointer crashes
2. Force explicit handling
3. Enable safe composition with iterators
4. Allow zero-cost abstraction

Key idea:

Option<T> encodes *possibility of absence* inside the type system.
