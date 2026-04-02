
# Rust Iterator & Ownership Mastery Notes (Full Session Transcript Summary)

This document captures the structured knowledge progression from the session.

---

# SECTION 1 — Option<T>

```rust
enum Option<T> {
    Some(T),
    None
}
```

map() transforms inner value.

and_then() chains Option-returning closures.

---

# SECTION 2 — Borrowing: String vs &str

String owns memory.

&str borrows memory.

trim() returns &str (no allocation)

to_string() allocates new String

---

# SECTION 3 — Lifetime Rule

References must not outlive owners.

Example invalid:

let a;
{
    let v = vec![1,2,3];
    a = v.iter().next();
}

Reason: vector dropped before reference used

---

# SECTION 4 — Iterator Model

Iterator trait:

trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

next() mutates iterator state (cursor), not collection

---

# SECTION 5 — iter vs iter_mut vs into_iter

iter() -> &T
iter_mut() -> &mut T
into_iter() -> T

---

# SECTION 6 — Adapter Closure Rules

map() receives Self::Item

filter() receives &Self::Item

find() receives &Self::Item

fold() receives (B, Self::Item)

---

# SECTION 7 — Example Pipeline Flow

iter() -> &i32
map() -> i32
filter closure receives &i32

---

# SECTION 8 — Why **x Appears

iter() produces &i32

find/filter closure receives &Self::Item

=> &&i32

**x removes both reference layers

---

# SECTION 9 — Temporary Lifetime Extension

Works:

let iter = vec![1,2,3].iter();

Compiler extends temporary lifetime internally

---

# SECTION 10 — Consumers vs Adapters

Adapters (lazy):

map filter take skip

Consumers (execute pipeline):

collect sum fold count find

---

# SECTION 11 — collect()

collect::<Vec<_>>() builds Vec<Self::Item>

Uses FromIterator trait

---

# SECTION 12 — sum()

sum::<i32>() returns i32

Uses Sum trait

Return type chosen by destination type

---

# SECTION 13 — fold()

fold(init, closure) -> init type

Closure receives (acc, item)

---

# SECTION 14 — fold Builds Containers

Example:

fold(Vec::new(), push loop) -> Vec<i32>

Equivalent to collect()

---

# SECTION 15 — Iterator Laziness

Nothing runs until consumer executes:

collect sum fold find

---

# SECTION 16 — Reference vs Owned Pipelines

iter() -> borrowed

into_iter() -> owned

---

# SECTION 17 — Example String fold()

Produces String from numeric iterator

---

# SECTION 18 — Type Inference Rule

Vec::new() inferred from push usage

Example:

acc.push(x)

=> Vec<i32>

---

# SECTION 19 — Closure Input ≠ Iterator Output

filter closure input = &Item

iterator output = Item

---

# SECTION 20 — Professional Iterator Reasoning Algorithm

1 determine Self::Item
2 check adapter signature
3 substitute Self::Item
4 update Self::Item if transformed

---

# SECTION 21 — Example Pipeline Evaluation

iter() -> &i32
map() -> i32
filter closure -> &i32
find closure -> &i32
return -> Option<i32>

---

# SECTION 22 — Lifetime-safe Reference Return

Valid:

fn first_large(v: &[i32]) -> Option<&i32>

Invalid:

fn bad() -> Option<&i32>

Reason: temporary dropped

---

# SECTION 23 — Iterator Cursor Model

Iterator = cursor

Collection = storage

next() mutates cursor only

---

# SECTION 24 — Reference Layer Rule

iter() adds one &

filter/find adds another &

closure gets &&T
