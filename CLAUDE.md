# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Building
```bash
cargo build                  # Standard build
cargo make build            # Build with cargo-make
cargo make build-release    # Release build
cargo make check            # Quick compile check
```

### Testing
```bash
cargo test                    # Run all tests
cargo test kata1             # Run tests for specific kata
cargo test temperature_tests # Run specific test module
cargo make test              # Run tests with cargo-make
cargo make test-kata -- kata1 # Run specific kata tests
```

### Code Quality
```bash
cargo clippy                 # Run linter with custom settings
cargo fmt                   # Format code with project configuration
cargo make clippy           # Run clippy with warnings as errors
cargo make format           # Check formatting
cargo make format-fix       # Auto-fix formatting issues
cargo make clippy-fix       # Auto-fix clippy issues
cargo make qa               # Run all quality checks (format, clippy, test)
cargo make qa-fix           # Fix issues and run checks
```

### Kata Management (with cargo-make)
```bash
cargo make list-katas           # Show all katas and their enabled status
cargo make enable-kata -- kata2 # Enable specific kata in main.rs
cargo make disable-kata -- kata2 # Disable specific kata in main.rs
```

### Utilities
```bash
cargo make clean            # Clean build artifacts
cargo make run              # Run the main binary
cargo make rebuild          # Clean and rebuild
```

### Dependencies
- `regex = "1"` - Used in kata4 for input validation
- `chrono = "0.4"` - Used in kata6 for timestamp formatting in logging

### Prerequisites
- **cargo-make**: Install with `cargo install cargo-make` for enhanced build tasks

## Project Architecture

This is a collection of 8 Rust clean code katas, each demonstrating specific principles:

1. **kata1_temperature_converter.rs** - Type safety with enums, single responsibility
2. **kata2_string_processor.rs** - Function extraction, avoiding repetition
3. **kata3_simple_calculator.rs** - Error handling with Result types and custom errors
4. **kata4_user_validation.rs** - Newtype pattern for strong typing and validation
5. **kata5_shape_area_calculator.rs** - Traits for open/closed principle
6. **kata6_simple_logging_system.rs** - Dependency injection and strategy pattern
7. **kata7_mini_todo_application.rs** - Clean architecture with domain/use case/infrastructure layers
8. **kata8_simple_parser.rs** - Parser combinators for CSV and JSON

### Key Architectural Patterns

**Single File Organization**: Each kata is completely self-contained in one file with embedded tests using `#[cfg(test)]` modules.

**Modular Structure**: All katas are consolidated into `rust_clean_code_katas.rs` rather than separate modules. Main.rs only imports kata1 (others commented out).

**Clean Architecture (Kata 7)**: The TODO application demonstrates proper layered architecture:
- `domain` module: Core business entities (Task, TaskId, Priority)
- `use_cases` module: Application logic (AddTaskUseCase, CompleteTaskUseCase)
- `ports` module: Interface definitions (TaskRepository trait)
- `infrastructure` module: Concrete implementations (InMemoryTaskRepository)
- `todo_app` module: Application facade

**Error Handling Patterns**: Custom error enums with Display and Error trait implementations across multiple katas (Calculator, Validation, Parser).

**Type Safety Patterns**: Extensive use of newtypes (Username, Email, Password, TaskId) and enums for type-safe APIs.

## Configuration

**Clippy Settings** (clippy.toml):
- MSRV: 1.80.0
- Cognitive complexity threshold: 15
- Max arguments: 5
- Max lines per function: 80

**Rustfmt Settings** (rustfmt.toml):
- Edition 2021, unstable features enabled
- Hard tabs (4 spaces), Unix line endings
- Max width: 100 characters
- Import organization: crate-level granularity, mixed layout
- Code formatting in doc comments enabled

## Working with Individual Katas

To enable a specific kata, uncomment its module declaration in main.rs:
```rust
mod kata2_string_processor;  // Uncomment to enable
```

Each kata includes comprehensive unit tests that demonstrate expected behavior and can serve as usage examples.