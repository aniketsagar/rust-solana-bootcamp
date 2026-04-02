# Rust Mastery Track — Concept 2: Borrowing and &str vs String

## Core Idea

Rust separates ownership from borrowing.

```
String → owns memory
&str   → borrows memory
```

---

## trim() Returns &str

Example:

```rust
let x = " hello ".trim();
```

Returns slice referencing original string memory.

---

## to_string()

Creates owned copy:

```rust
let x = " hello ".trim().to_string();
```

Allocates new memory.

---

## Lifetime Safety Insight

References must not outlive their source.

Example (fails):

```rust
let a;

{
    let v = vec![1,2,3];
    a = v.iter().next();
}
```

Reason:

```
vector dropped
reference invalid
```

---

## Exercises

### Exercise 1

Return first word slice:

```rust
fn first_word_slice(s: &str) -> Option<&str>
```

### Exercise 2

Return last word slice:

```rust
fn last_word_slice(s: &str) -> Option<&str>
```

### Exercise 3

Return trimmed string length without allocation

```rust
fn trimmed_len(s: &str) -> usize
```

---

## Key Insights Learned

- &str is a view into memory
- String owns memory
- trim() returns slice
- references cannot outlive owners
