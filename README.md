# Tour of Rust

A collection of Rust examples organized by chapters as a Cargo workspace.

## Structure

The project is organized as a Cargo workspace with chapters:

### Chapter 1 - Basics
Located in `chapter1/`:
- `hello.rs` - Basic Hello World
- `variables.rs` - Variable declarations and types
- `basic_type_convertion.rs` - Basic type conversion
- `constants.rs` - Constants in Rust
- `basic_types.rs` - Basic types in Rust
- `changing_variables.rs` - Mutable variables
- `if_else.rs` - Conditional statements
- `loop.rs` - Infinite loops with break
- `while.rs` - While loops
- `for.rs` - For loops with ranges
- `functions.rs` - Functions with explicit and implicit returns
- `multiple_return_values.rs` - Returning tuples from functions
- `returning_nothing.rs` - Functions returning unit type ()
- `returning_values_from_block_expressions.rs` - Returning values from blocks, if, and match
- `match.rs` - Pattern matching with match
- `return_values_from_loop.rs` - Breaking with values from loops
- `calling_methods.rs` - Calling static methods and instance methods
- `creating_data_in_memory.rs` - Stack vs heap data in structs
- `result.rs` - Using Result for error handling
- `failable_main.rs` - Main function that can return errors
- `graceful_error_handling.rs` - Using ? operator for error propagation
- `unwrap.rs` - Using unwrap (panics on error)
- `option.rs` - Using Option type with generics
- `representing_nothing.rs` - Custom enum for representing nothing

### Chapter 2 - Data Structures
Located in `chapter2/`:
- `structures.rs` - Defining and using structs
- `tuple-like_structs.rs` - Tuple structs
- `unit-like_structs.rs` - Unit-like structs
- `vectors.rs` - Working with Vec<T>
- `enumerations.rs` - Basic enums and pattern matching
- `enumerations_with_data.rs` - Enums with associated data
- `generic_types.rs` - Generic type parameters

### Chapter 3 - Ownership and Lifetimes
Located in `chapter3/`:
- `ownership.rs` - Basic ownership concepts
- `moving_ownership.rs` - Moving ownership to functions
- `returning_ownership.rs` - Returning ownership from functions
- `borrowing_ownership_with_references.rs` - Borrowing with references
- `borrowing_mutable_ownership_with_references.rs` - Mutable borrowing
- `passing_around_borrowed_data.rs` - Modifying borrowed data
- `dereferencing.rs` - Using dereference operator
- `scope-based_resource_management.rs` - Scope-based cleanup
- `dropping_is_hierarchical.rs` - Hierarchical dropping
- `static_lifetimes.rs` - Static lifetime and variables
- `explicit_lifetimes.rs` - Explicit lifetime annotations
- `multiple_lifetimes.rs` - Multiple lifetime parameters
- `references_of_references.rs` - References to references
- `lifetimes_in_data_types.rs` - Using lifetimes in struct definitions

## Running examples

Navigate to the chapter directory and run examples:

```bash
# Chapter 1
cd chapter1 && cargo run --bin hello
cargo run --bin variables
# ... other chapter1 examples

# Chapter 2
cd ../chapter2 && cargo run --bin structures
cargo run --bin vectors
# ... other chapter2 examples

# Chapter 3
cd ../chapter3 && cargo run --bin ownership
cargo run --bin static_lifetimes
# ... other chapter3 examples
```

Or from the root:
```bash
cargo run -p tour_of_rust_chapter1 --bin hello
cargo run -p tour_of_rust_chapter2 --bin structures
cargo run -p tour_of_rust_chapter3 --bin ownership
```
