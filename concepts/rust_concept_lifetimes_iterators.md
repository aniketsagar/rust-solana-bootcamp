# Rust Mastery Track — Concept 4: Lifetimes Inside Iterator Pipelines

## Core Idea

Iterators often produce **references**, not values.

Understanding when references are valid depends on **lifetimes**.

Key rule:

```
references must not outlive their owner
```

---

# iter() vs into_iter()

Example:

```rust
let v = vec![1,2,3];

v.iter()        // Item = &i32
v.into_iter()   // Item = i32
```

Meaning:

```
iter() borrows
into_iter() moves ownership
```

---

# Why Vec<&T> Works

Example:

```rust
let v = vec![10,20,30];

let result = v.iter()
    .filter(|x| **x > 10)
    .collect::<Vec<&i32>>();
```

Works because:

```
references point into v
v still alive
```

Memory model:

```
v owns data
result borrows data
```

---

# Why This Sometimes Fails

Example:

```rust
let result = vec![10,20,30]
    .iter()
    .collect::<Vec<&i32>>();
```

Fails because:

```
temporary vector dropped immediately
references become invalid
```

Rust prevents dangling references.

---

# Temporary Lifetime Extension Rule

This works:

```rust
let iter = vec![1,2,3].iter();
```

Rust internally rewrites roughly as:

```
let temp = vec![1,2,3];
let iter = temp.iter();
```

So:

```
temp lives long enough
```

---

# Closure Reference Flow Rule

Example:

```rust
v.iter().filter(|x| **x > 10)
```

Pipeline:

```
iter()  -> &i32
filter -> receives &&i32
```

Example:

```rust
v.iter().map(|x| *x)
```

Pipeline:

```
iter() -> &i32
map()  -> i32
```

Key insight:

```
closure input type != iterator output type
```

---

# Returning References From Iterators

Example:

```rust
fn first_large(v: &[i32]) -> Option<&i32> {
    v.iter().find(|x| **x > 10)
}
```

Works because:

```
slice lives longer than returned reference
```

---

# Example That Fails

Example:

```rust
fn bad() -> Option<&i32> {
    vec![1,2,3].iter().next()
}
```

Fails because:

```
vector destroyed before return
reference invalid
```

Rust prevents dangling references.

---

# fold() With References

Example:

```rust
v.iter().fold(Vec::new(), |mut acc, x| {
    acc.push(x);
    acc
})
```

Produces:

```
Vec<&i32>
```

Because:

```
x = &i32
```

---

# collect() Lifetime Behavior

Example:

```rust
v.iter().collect::<Vec<&i32>>()
```

Produces:

```
Vec<&i32>
```

But:

```rust
v.into_iter().collect::<Vec<&i32>>()
```

Fails because:

```
items moved
references unavailable
```

---

# Key Lifetime Insight

Rule:

```
iter()  → produces references
into_iter() → produces owned values
```

Choose based on:

```
do I want ownership or borrowing?
```

---

# Exercises

## Exercise 1

Explain why this works:

```rust
fn example(v: &[i32]) -> Vec<&i32> {
    v.iter().filter(|x| **x > 10).collect()
}
```

---

## Exercise 2

Fix this function:

```rust
fn broken() -> Vec<&i32> {
    vec![1,2,3].iter().collect()
}
```

---

## Exercise 3

Write function returning first even number reference:

```rust
fn first_even_ref(v: &[i32]) -> Option<&i32>
```

---

## Exercise 4

Write function returning owned even numbers:

```rust
fn even_values(v: Vec<i32>) -> Vec<i32>
```

Hint:

```
into_iter()
filter()
collect()
```

---

# Key Insights Learned

- references must not outlive owners
- iter() produces references
- into_iter() produces owned values
- collect() preserves iterator item type
- temporary values cannot safely return references
