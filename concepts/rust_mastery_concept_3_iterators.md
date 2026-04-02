# Rust Mastery Track – Concept 3

## Iterator Theory

---

## What Is an Iterator?

An iterator is:

a lazy sequence of values produced one at a time

Example:

```rust
let v = vec![1, 2, 3];
let mut iter = v.iter();

iter.next(); // Some(&1)
iter.next(); // Some(&2)
iter.next(); // Some(&3)
iter.next(); // None
```

---

## Iterator Trait (Core Definition)

Every iterator implements:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Meaning:

Return next value OR None

---

## Three Types of Iteration

Given:

```rust
let v = vec![10, 20, 30];
```

### iter()

```rust
v.iter()
```

Produces:

```
Iterator<&i32>
```

Does NOT consume vector

---

### iter_mut()

```rust
v.iter_mut()
```

Produces:

Iterator<&mut i32>

Allows modification

---

### into_iter()

```rust
v.into_iter()
```

Produces:

Iterator<i32>

Consumes vector

---

## Iterator Pipelines

Example:

```rust
nums.iter().filter(|x| *x > 5).map(|x| x * 2)
```

Steps:

borrow → filter → transform

Still lazy until consumed

---

## Lazy Execution

This does nothing yet:

```rust
nums.iter().map(|x| x * 2);
```

Execution starts only when consumed:

Examples:

- collect()
- find()
- sum()
- count()
- next()

---

## Most Important Iterator Functions

| Function  | Purpose              |
| --------- | -------------------- |
| map()     | transform values     |
| filter()  | keep matching values |
| find()    | first matching value |
| any()     | check if any match   |
| all()     | check if all match   |
| collect() | build collection     |
| count()   | count items          |
| last()    | last item            |
| next()    | next item            |

---

## Example: First Even Number

```rust
fn first_even(nums: &[i32]) -> Option<i32> {
    nums.iter()
        .find(|x| *x % 2 == 0)
        .copied()
}
```

Type flow:

Iterator<&i32>
→ Option<&i32>
→ Option<i32>

---

## Key Takeaways

Iterators are:

- lazy
- memory efficient
- composable
- zero-cost abstractions

Mastering iterators = mastering Rust pipelines.
