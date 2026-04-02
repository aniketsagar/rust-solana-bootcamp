
# Rust Mastery — Concept 1 (Deep) Part 2
## map() vs and_then()

map():

Option<T> -> Option<U>

Example:

Some("rust").map(|x| x.len())

and_then():

Option<T> -> Option<U>

but used when closure returns Option already

Example:

Some("rust").and_then(|x| Some(x.len()))

Rule:

map() wraps
and_then() flattens
