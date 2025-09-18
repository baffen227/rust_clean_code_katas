# Project Overview

This project is a collection of Rust code katas, each designed to illustrate a specific clean code principle. The katas are self-contained within their own files and cover topics such as type safety, error handling, design patterns, and architectural principles. Each kata includes its own set of unit tests.

The code is written in Rust and includes comments in English.

## Katas

1.  **Temperature Converter:** Demonstrates good naming, single responsibility, and type safety.
2.  **String Processor:** Focuses on extracting small functions and avoiding repetition.
3.  **Simple Calculator:** Highlights error handling with `Result` and custom errors.
4.  **User Validation System:** Uses the newtype pattern for strong types and validation.
5.  **Shape Area Calculator:** Employs the open/closed principle and traits.
6.  **Simple Logging System:** Shows dependency injection and the strategy pattern.
7.  **Mini TODO Application:** Implements a clean architecture with domain, use case, and infrastructure layers.
8.  **Simple Parser:** Uses a combinator pattern for parsing CSV and JSON.

# Building and Running

To build and run the code, you will need to have the Rust toolchain and `cargo` installed.

First, create a new cargo project:

```sh
cargo new rust_clean_code_katas_project
cd rust_clean_code_katas_project
```

Then, move the kata files into the `src` directory.

Next, you'll need to declare the modules in `src/lib.rs`:

```rust
pub mod kata1_temperature_converter;
pub mod kata2_string_processor;
pub mod kata3_simple_calculator;
pub mod kata4_user_validation;
pub mod kata5_shape_area_calculator;
pub mod kata6_simple_logging_system;
pub mod kata7_mini_todo_application;
pub mod kata8_simple_parser;
```

## Building

To build the project, you can use the following command:

```sh
cargo build
```

## Testing

To run the tests for all the katas, use the following command:

```sh
cargo test
```

# Development Conventions

*   **File Structure:** Each kata is in its own file within the `src` directory.
*   **Testing:** Each kata has its own test module, which is co-located with the code.
*   **Comments:** The code is commented in English.
