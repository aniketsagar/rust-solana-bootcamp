
# Rust Mastery Track – Concept 1
## Option<T> + Pattern Matching + Pipelines

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

value exists  → Some(value)
value missing → None

This forces safe handling of missing values at compile time.

---

## Mental Model

Think:

Option<T> = safe maybe-value

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

## Core Pattern: Pattern Matching with Option

```rust
match option_value {
    Some(x) => ...
    None => ...
}
```

More concise alternatives:

```rust
if let Some(x) = option_value
```

Pipeline style:

```rust
option.map(...)
option.and_then(...)
```

---

## Option Mapping

Use map() when transforming:

T → U

Example:

```rust
Some("hello").map(|x| x.len())
```

Result:

Some(5)

General rule:

Option<T>.map(T → U) = Option<U>

---

## Option Chaining with and_then()

Use and_then() when transforming:

T → Option<U>

Example:

```rust
Some("hello").and_then(|x| x.chars().last())
```

Result:

Some('o')

General rule:

Option<T>.and_then(T → Option<U>) = Option<U>

---

## map() vs and_then()

Function returns | Use
-----------------|-----
U | map()
Option<U> | and_then()

Example comparison:

```rust
Some("rust").map(|x| x.len())
```

Result:

Some(4)

```rust
Some("rust").map(|x| x.chars().last())
```

Result:

Some(Some('t'))

```rust
Some("rust").and_then(|x| x.chars().last())
```

Result:

Some('t')

---

## Example Problem 1: First Even Number

Return the first even number from a vector.

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

---

## Example Problem 2: First Word

```rust
fn first_word(s: &str) -> Option<String> {
    s.split_whitespace()
        .next()
        .map(|x| x.to_string())
}
```

Pattern:

Iterator → Option<&str> → Option<String>

---

## Example Problem 3: Last Character

```rust
fn last_char(s: &str) -> Option<char> {
    s.split_whitespace()
        .last()
        .and_then(|word| word.chars().last())
}
```

Pattern:

Option<&str> → Option<char>

---

## Example Problem 4: Last Word Length

```rust
fn last_word_length(s: &str) -> Option<usize> {
    s.split_whitespace()
        .last()
        .map(|word| word.len())
}
```

Pattern:

Option<&str> → Option<usize>

---

## Key Takeaways

Memorize these rules:

map()      → use when function returns U
and_then() → use when function returns Option<U>

Pipeline thinking:

input → iterator → Option → transform safely

This pattern appears everywhere in production Rust:

- parsing
- configuration loading
- backend services
- Solana programs
- CLI tools
