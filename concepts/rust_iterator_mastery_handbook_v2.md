# Rust Iterator Mastery Handbook — Version 2
## Personalized Study Track (Based on Your Session Progress)

This handbook consolidates:
- iterator mental models
- closure parameter inference rules
- borrowing vs ownership behavior
- lifetime-safe pipelines
- fold / collect / sum internals
- progressive exercises
- mastery checklist ladder

---
# SECTION 1 — Core Iterator Mental Model

Iterator trait:

```
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Key insight:

```
Iterator = cursor
Collection = storage
next() mutates cursor, not collection
```

---
# SECTION 2 — iter vs iter_mut vs into_iter

| Method | Produces | Ownership |
|-------|----------|-----------|
| iter() | &T | borrow |
| iter_mut() | &mut T | mutable borrow |
| into_iter() | T | move |

Rule:

```
iter() adds one reference layer
```

---
# SECTION 3 — Adapter Closure Input Rules (CRITICAL)

| Adapter | Closure receives |
|--------|-----------------|
| map() | Self::Item |
| filter() | &Self::Item |
| find() | &Self::Item |
| any() | &Self::Item |
| all() | &Self::Item |
| fold() | (B, Self::Item) |

Master rule:

```
Closure input type ≠ iterator output type
```

---
# SECTION 4 — Reference Layer Rule

Pipeline:

```
iter() -> &T
filter() -> &&T
map(|x| *x) -> removes layer
```

Example:

```
v.iter().find(|x| **x > 10)
```

Because:

```
iter() → &T
find() → &&T
```

---
# SECTION 5 — Professional Iterator Reasoning Algorithm

Always apply:

```
1 determine Self::Item
2 check adapter signature
3 substitute Self::Item
4 update Self::Item if adapter transforms
```

This is your primary debugging tool.

---
# SECTION 6 — Consumers vs Adapters

Adapters (lazy):

```
map filter take skip enumerate
```

Consumers (execute pipeline):

```
collect sum fold count find any all
```

Execution begins only at consumer.

---
# SECTION 7 — collect()

Signature:

```
collect<B>() -> B
where B: FromIterator<Self::Item>
```

Example:

```
.collect::<Vec<_>>()
```

Container determined by destination type.

---
# SECTION 8 — sum()

Signature:

```
sum<S>() -> S
where S: Sum<Self::Item>
```

Return type inferred from context.

Example:

```
.sum::<i32>()
```

---
# SECTION 9 — fold()

Signature:

```
fold<B>(init: B, closure) -> B
```

Closure receives:

```
(B, Self::Item)
```

Accumulator controls output type.

Example:

```
fold(0, |acc, x| acc + x)
```

Equivalent to:

```
sum()
```

---
# SECTION 10 — Temporary Lifetime Extension

Works:

```
let iter = vec![1,2,3].iter();
```

Compiler internally:

```
let temp = vec![1,2,3];
let iter = temp.iter();
```

So references remain valid.

---
# SECTION 11 — Returning References Safely

Valid:

```
fn first_large(v: &[i32]) -> Option<&i32>
```

Invalid:

```
fn bad() -> Option<&i32>
```

Reason:

```
temporary dropped too early
```

---
# SECTION 12 — fold Builds Containers

Example:

```
v.iter().fold(Vec::new(), |mut acc, x| {
    acc.push(*x);
    acc
})
```

Equivalent to:

```
collect::<Vec<_>>()
```

---
# SECTION 13 — Lifetime Rule

Core law:

```
References must not outlive owners
```

---
# SECTION 14 — Type Inference Rule

Example:

```
Vec::new()
```

Later:

```
acc.push(x)
```

Rust infers:

```
Vec<i32>
```

---
# SECTION 15 — Progressive Practice Ladder

## Level 1

```
first_even(nums: &[i32]) -> Option<i32>
```

## Level 2

```
last_word_length(s: &str) -> Option<usize>
```

## Level 3

```
collect squares > 10
```

```
fn squares(nums: &[i32]) -> Vec<i32>
```

## Level 4

Return borrowed references safely:

```
fn evens_ref(v: &[i32]) -> Vec<&i32>
```

## Level 5

Return owned values via into_iter():

```
fn evens_owned(v: Vec<i32>) -> Vec<i32>
```

## Level 6

Fold-based collector:

```
fn concat(nums: &[i32]) -> String
```

## Level 7

Manual collect implementation using fold:

```
fn manual_collect(v: &[i32]) -> Vec<i32>
```

---
# SECTION 16 — Iterator Fluency Checklist

You can now:

✔ track Self::Item across pipelines  
✔ identify closure parameter types  
✔ distinguish adapters vs consumers  
✔ predict collect() container types  
✔ infer fold() accumulator behavior  
✔ reason about temporary lifetime extension  
✔ safely return references from iterators  

Next milestone:

```
explicit lifetime parameters inside iterator-returning functions
```

