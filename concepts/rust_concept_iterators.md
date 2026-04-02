# Rust Mastery Track — Concept 3: Iterators Deep Dive

## Core Iterator Model

Iterator trait:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Key insight:

```
next() mutates iterator state
NOT underlying collection
```

---

## iter vs iter_mut vs into_iter

| Method | Produces |
|--------|----------|
| iter() | &T |
| iter_mut() | &mut T |
| into_iter() | T |

---

## Adapter Types

### map()

Receives:

```
Self::Item
```

Example:

```rust
v.iter().map(|x| *x + 1)
```

---

### filter()

Receives:

```
&Self::Item
```

Example:

```rust
v.iter().filter(|x| **x > 10)
```

---

### find()

Receives:

```
&Self::Item
```

Returns:

```
Option<Self::Item>
```

---

## Consumers

End iterator pipelines.

Examples:

```
collect()
sum()
fold()
count()
```

---

## fold()

Signature:

```rust
fold(init, |acc, item| ...)
```

Accumulator type controls result.

Example:

```rust
v.iter().fold(0, |acc, x| acc + x)
```

---

## collect()

Converts iterator into container:

```rust
.collect::<Vec<_>>()
```

Uses FromIterator trait.

---

## sum()

Signature:

```rust
sum<S>() -> S
```

Return type inferred.

Example:

```rust
v.iter().sum::<i32>()
```

---

## Exercises

### Exercise 1

Return first even number

```rust
fn first_even(nums: &[i32]) -> Option<i32>
```

### Exercise 2

Collect squares > 10

```rust
fn squares_gt_10(nums: &[i32]) -> Vec<i32>
```

### Exercise 3

Concatenate numbers into string using fold

```rust
fn concat(nums: &[i32]) -> String
```

---

## Key Insights Learned

- iterators are lazy
- adapters transform pipelines
- consumers execute pipelines
- fold is universal reducer
