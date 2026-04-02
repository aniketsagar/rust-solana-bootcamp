
# Rust Mastery Track – Concept 2
## Borrowing vs Ownership (`String` vs `&str`)

---

## Core Idea

Rust separates:

String = owns heap memory
&str   = borrowed slice view into memory

Example:

```rust
let s = String::from("hello");
let slice = &s[..];
```

No copy happens. `slice` points to the same memory.

---

## Why Functions Prefer `&str`

Prefer:

```rust
fn print_name(name: &str)
```

instead of:

```rust
fn print_name(name: String)
```

Because `&str`:

- avoids copying
- avoids allocation
- works with String, &String, &str, and literals
- improves performance

---

## Zero-Copy Programming

Many Rust string operations return `&str` instead of `String`.

Examples:

| Operation | Returns |
|----------|---------|
| trim() | &str |
| split_whitespace() | iterator of &str |
| lines() | iterator of &str |
| split(',') | iterator of &str |

Reason:

They change *view boundaries*, not contents.

---

## When Rust Returns `String`

If content changes → new allocation required.

Examples:

| Operation | Returns |
|----------|---------|
| to_uppercase() | String |
| replace() | String |
| format!() | String |

Rule:

Change view → &str  
Change content → String

---

## Example: First Character Without Allocation

```rust
fn first_char(s: &str) -> Option<char> {
    s.trim().chars().next()
}
```

Pipeline:

&str → trim() → chars() → next()

No allocation happens.

---

## Example: Last Non-Space Character

```rust
fn last_non_space_char(s: &str) -> Option<char> {
    s.trim().chars().rev().next()
}
```

Efficient because:

- trims boundaries
- iterates from end
- stops early

---

## Returning Borrowed Data From Functions

Example:

```rust
fn first_word_slice(s: &str) -> Option<&str> {
    s.trim().split_whitespace().next()
}
```

Benefits:

- zero-copy
- fast
- idiomatic Rust API style

---

## Key Takeaways

Prefer `&str` unless ownership is required.

Borrow chains like:

input → trim() → split() → next()

are allocation-free and common in production Rust.
