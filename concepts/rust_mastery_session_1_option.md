
# Rust Mastery Track – Session 1
## Concept: Option<T> + Pattern Matching

---

## What is Option<T>?

Rust replaces `null` with:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Meaning:

```
value exists  → Some(value)
value missing → None
```

---

## Mental Model

Think:

```
Option<T> = safe maybe-value
```

Example:

```rust
fn find_even(n: i32) -> Option<i32> {
    if n % 2 == 0 {
        Some(n)
    } else {
        None
    }
}
```

Usage:

```rust
match find_even(4) {
    Some(x) => println!("Found {}", x),
    None => println!("Not even"),
}
```

---

## Core Pattern

Pattern matching with Option:

```rust
match option_value {
    Some(x) => ...
    None => ...
}
```

Later becomes:

```rust
if let Some(x) = option_value
```

Later becomes:

```rust
option.map(...)
```

Later becomes:

```rust
option.unwrap_or(...)
```

Same concept. Increasing abstraction level.

---

## Example Problem (Solved)

Return the first even number from a vector.

If none exists → return None

Solution:

```rust
fn first_even(nums: Vec<i32>) -> Option<i32> {
    for n in nums {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}
```

Usage:

```rust
match first_even(vec![1,3,5,8,9]) {
    Some(n) => println!("First even: {}", n),
    None => println!("No even number"),
}
```

---

## Level 1 Challenge (Your Turn)

Write:

```rust
fn first_word(s: &str) -> Option<String>
```

Return:

```
Some(first_word)
```

or

```
None
```

if string is empty or only spaces.

Examples:

```
"hello world" → Some("hello")
"rust" → Some("rust")
"   " → None
"" → None
```

Constraints:

Use at least one of:

```
match
if let
split_whitespace
```
