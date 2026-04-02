
# Concept 1 — Option<T> Deep Chapter (Part 2)
## Option<T> as Control Flow Algebra

Instead of writing:

match maybe_value {
    Some(x) => process(x),
    None => handle_missing()
}

Rust lets you write pipelines:

maybe_value.map(process)

This transforms:

Option<T> → Option<U>

So Option acts like:

a computation context that may fail safely
