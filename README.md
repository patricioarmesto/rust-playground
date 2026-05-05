# Tour of Rust

A collection of Rust examples organized as independent binaries.

## Structure

Each concept is a separate binary in `src/bin/`:
- `hello.rs` - Basic Hello World
- `variables.rs` - Variable declarations and types
- `basic_type_convertion.rs` - Basic type conversion
- `constants.rs` - Constants in Rust
- `arrays.rs` - Arrays and type annotations
- `functions.rs` - Functions with explicit and implicit returns
- `multiple_return_values.rs` - Returning tuples from functions
- `basic_types.rs` - Basic types in Rust
- `changing_variables.rs` - Mutable variables
- `if_else.rs` - Conditional statements
- `loop.rs` - Infinite loops with break
- `while.rs` - While loops
- `returning_nothing.rs` - Functions returning unit type ()
- `for.rs` - For loops with ranges
- `match.rs` - Pattern matching with match
- `return_values_from_loop.rs` - Breaking with values from loops
- `returning_values_from_block_expressions.rs` - Returning values from blocks, if, and match
- `structures.rs` - Defining and using structs
- `calling_methods.rs` - Calling static methods and instance methods
- `creating_data_in_memory.rs` - Stack vs heap data in structs
- `enumerations.rs` - Basic enums and pattern matching
- `enumerations_with_data.rs` - Enums with associated data
- `generic_types.rs` - Generic type parameters
- `option.rs` - Using Option type with generics
- `representing_nothing.rs` - Custom enum for representing nothing
- `tuple-like_structs.rs` - Tuple structs
- `unit-like_structs.rs` - Unit-like structs

## Running examples

```bash
cargo run --bin hello
cargo run --bin variables
cargo run --bin basic_type_convertion
cargo run --bin constants
cargo run --bin arrays
cargo run --bin functions
cargo run --bin multiple_return_values
cargo run --bin basic_types
cargo run --bin changing_variables
cargo run --bin if_else
cargo run --bin loop
cargo run --bin while
cargo run --bin returning_nothing
cargo run --bin for
cargo run --bin match
cargo run --bin return_values_from_loop
cargo run --bin returning_values_from_block_expressions
cargo run --bin structures
cargo run --bin calling_methods
cargo run --bin creating_data_in_memory
cargo run --bin enumerations
cargo run --bin enumerations_with_data
cargo run --bin generic_types
cargo run --bin option
cargo run --bin representing_nothing
cargo run --bin tuple-like_structs
cargo run --bin unit-like_structs
```
