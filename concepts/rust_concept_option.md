# Rust Mastery Track — Concept 1: Option<T>

## Core Idea

`Option<T>` represents presence or absence of a value safely.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Used instead of null pointers.

---

## Important Methods

| Method | Purpose |
|-------|---------|
| map() | Transform value if present |
| and_then() | Chain Option-returning functions |
| unwrap() | Extract value (unsafe if None) |
| unwrap_or() | Provide default |
| is_some() | Check presence |
| is_none() | Check absence |

---

## map vs and_then

```rust
Some("rust").map(|x| x.len())   // Some(4)
Some("rust").and_then(|x| Some(x.len())) // Some(4)
```

Difference:

```
map:      Option<T> -> Option<U>
and_then: Option<T> -> Option<Option<U>> flattened to Option<U>
```

---

## Pattern: Safe Pipelines

Example:

```rust
fn last_word_length(s: &str) -> Option<usize> {
    s.split_whitespace().last().map(|w| w.len())
}
```

---

## Exercises

### Exercise 1

Write:

```rust
fn first_char(s: &str) -> Option<char>
```

### Exercise 2

Write:

```rust
fn last_char(s: &str) -> Option<char>
```

### Exercise 3

Write:

```rust
fn second_word(s: &str) -> Option<&str>
```

---

## Key Insights Learned

- Option replaces null safely
- map transforms values
- and_then chains fallible operations
- Pipelines reduce nested match expressions
