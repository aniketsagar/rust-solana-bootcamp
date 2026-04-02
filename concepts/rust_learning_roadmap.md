# Rust Learning Roadmap (Structured Progression)

This roadmap represents the structured progression we will follow across
all Rust learning sessions. It is designed for transitioning from
*writing Rust* to *thinking in Rust*.

------------------------------------------------------------------------

## Phase 0 --- Mental Model Shift (Completed)

-   Ownership vs borrowing intuition
-   Immutable vs mutable references
-   Dereferencing iterator items
-   Pattern matching with `match`
-   `Option<T>` as control-flow algebra
-   Accumulator reasoning inside `fold`
-   Closure parameter inference
-   Container inference from iterator consumers
-   `Self::Item` transitions across iterator chains

------------------------------------------------------------------------

## Phase 1 --- Iterator Type-Flow Mastery (Current Phase)

-   Iterator algebra pipelines
-   Tracking `Self::Item` evolution
-   Consumer selection logic
-   Short-circuit vs non-short-circuit iterators
-   Returning references from iterator closures

------------------------------------------------------------------------

## Phase 2 --- Lifetime Flow Through Iterators

-   Lifetime propagation inside closures
-   Lifetime elision rules in iterator pipelines
-   Borrow checker reasoning inside iterator chains
-   `iter()` vs `iter_mut()` vs `into_iter()`

------------------------------------------------------------------------

## Phase 3 --- Iterator Ownership Transitions

-   Iterator ownership triangle
-   Moving vs borrowing inside closures
-   Closure pattern matching
-   `copied()` vs `cloned()` vs `to_owned()`

------------------------------------------------------------------------

## Phase 4 --- Option / Result Control-Flow Algebra

-   Option chaining (`map`, `and_then`, etc.)
-   Result chaining
-   `?` operator mastery
-   Converting between Option and Result

------------------------------------------------------------------------

## Phase 5 --- Pattern Matching as a Programming Language

-   Advanced match expressions
-   Match guards
-   Nested destructuring
-   `if let` / `while let`
-   `let-else` expressions

------------------------------------------------------------------------

## Phase 6 --- Struct + Enum Design Thinking

-   Designing expressive enums
-   Domain modeling with enums
-   Data-oriented API design

------------------------------------------------------------------------

## Phase 7 --- Borrow Checker Deep Fluency

-   Multiple immutable vs single mutable borrow reasoning
-   Non-lexical lifetimes
-   Reborrowing transitions

------------------------------------------------------------------------

## Phase 8 --- Closures as First-Class Compiler Objects

-   Closure capture modes (`Fn`, `FnMut`, `FnOnce`)
-   Move closures
-   Closure type inference

------------------------------------------------------------------------

## Phase 9 --- Trait Thinking (Professional Rust Boundary)

-   Trait bounds
-   Generic functions
-   Associated types

------------------------------------------------------------------------

## Phase 10 --- Iterator Internals

-   Zero-cost abstractions
-   Adapter vs consumer architecture
-   Writing custom iterators

------------------------------------------------------------------------

## Phase 11 --- Error Handling Architecture

-   Designing Result-based APIs
-   Custom error enums
-   `thiserror` and `anyhow` usage reasoning

------------------------------------------------------------------------

## Phase 12 --- Smart Pointers

-   Box
-   Rc
-   Arc
-   RefCell
-   Interior mutability

------------------------------------------------------------------------

## Phase 13 --- Module Architecture

-   crate vs module
-   `pub` vs `pub(crate)`
-   Workspace layout

------------------------------------------------------------------------

## Phase 14 --- Real Parsing Pipelines

-   Streaming parsing with iterators
-   Tokenizer architecture
-   Zero-copy parsing using `&str` slicing

------------------------------------------------------------------------

## Phase 15 --- Async Rust Foundations

-   Future model
-   async/await state machines
-   Send vs Sync

------------------------------------------------------------------------

## Phase 16 --- Performance Thinking

-   Allocation awareness
-   Stack vs heap control
-   Cache-friendly iteration
