# 🦀 Rust Clean Code Katas

[![Rust](https://img.shields.io/badge/rust-nightly--2025--01--01-orange.svg)](https://www.rust-lang.org/)
[![Nix](https://img.shields.io/badge/nix-shell-blue.svg)](https://nixos.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A collection of **8 hands-on coding exercises** designed to help you master **clean code principles** in Rust. Each kata focuses on specific architectural patterns, design principles, and best practices that make Rust code more maintainable, readable, and robust.

## 🎯 What You'll Learn

- **Type Safety**: Leveraging Rust's type system for bulletproof APIs
- **Error Handling**: Idiomatic error management with `Result` types
- **Clean Architecture**: Separation of concerns and dependency injection
- **Design Patterns**: Strategy, Repository, and Factory patterns in Rust
- **Code Organization**: Single responsibility and open/closed principles
- **Testing**: Comprehensive unit testing with embedded test modules

## 🚀 Quick Start

### Prerequisites

- [Nix](https://nixos.org/download.html) package manager
- Git

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/rust_clean_code_katas.git
cd rust_clean_code_katas

# Verify your environment
./verify-shell.sh

# Enter the development environment
nix-shell

# Run a kata (inside nix-shell)
cargo run
```

## 🥋 The Katas

### 1. 🌡️ Temperature Converter
**Focus**: Type safety with enums, single responsibility principle

Learn how to use Rust's powerful enum system to create type-safe APIs that prevent runtime errors and make invalid states unrepresentable.

### 2. 📝 String Processor
**Focus**: Function extraction, avoiding code repetition

Practice breaking down complex functions into smaller, focused units while eliminating duplication through strategic function composition.

### 3. 🔢 Simple Calculator
**Focus**: Error handling with Result types and custom errors

Master Rust's error handling patterns by building a calculator that gracefully handles invalid operations and provides meaningful error messages.

### 4. ✅ User Validation
**Focus**: Newtype pattern for strong typing and validation

Explore the newtype pattern to create self-validating types that encode business rules directly in the type system.

### 5. 📐 Shape Area Calculator
**Focus**: Traits for open/closed principle

Implement the open/closed principle using Rust traits to create extensible systems that are open for extension but closed for modification.

### 6. 📊 Simple Logging System
**Focus**: Dependency injection and strategy pattern

Build a flexible logging system using dependency injection and the strategy pattern to support multiple output formats and destinations.

### 7. 📋 Mini Todo Application
**Focus**: Clean architecture with layered design

Construct a complete application following clean architecture principles with domain, use case, and infrastructure layers.

### 8. 🔍 Simple Parser
**Focus**: Parser combinators for CSV and JSON

Create robust parsers using combinator patterns to handle complex data formats with proper error recovery.

## 🛠️ Development Environment

This project uses **Nix** to provide a reproducible development environment with:

- **Rust nightly-2025-01-01** toolchain
- **cargo-make** for enhanced build tasks
- **Development utilities**: claude-code, gemini-cli, kiro, zed-editor
- **Proper LSP support** with rust-analyzer

### Environment Verification

```bash
./verify-shell.sh
```

Expected output:
```
🔍 Nix Shell Environment Healthcheck
=====================================

=== Shell Environment ===
  Testing shell entry... ✓ PASS

=== Rust Toolchain ===
  Testing Rust version... ✓ PASS
  Testing cargo... ✓ PASS
  Testing clippy... ✓ PASS
  Testing rustfmt... ✓ PASS

=== Required Tools ===
  Testing cargo-make... ✓ PASS

=== Unstable Utilities ===
  Testing claude-code... ✓ PASS
  Testing gemini-cli... ✓ PASS
  Testing kiro... ✓ PASS
  Testing zed-editor... ✓ PASS

=== Development Workflow ===
  Testing cargo check... ✓ PASS

🎉 All tests passed! Environment is ready for development.
```

## 📚 Working with Katas

### Enable a Kata

Each kata is self-contained in its own file. To work on a specific kata:

1. **Edit `src/main.rs`** - Uncomment the desired kata module:
   ```rust
   // mod kata1_temperature_converter;  // ← Uncomment this line
   // mod kata2_string_processor;
   // mod kata3_simple_calculator;
   // etc...
   ```

2. **Run the kata**:
   ```bash
   cargo run
   ```

3. **Run tests**:
   ```bash
   cargo test                    # All tests
   cargo test kata2             # Specific kata tests
   cargo test temperature_tests # Specific test module
   ```

### Development Commands

```bash
# Building
cargo build                  # Standard build
cargo make build            # Build with cargo-make
cargo make build-release    # Release build

# Testing
cargo test                  # Run all tests
cargo make test             # Run tests with cargo-make
cargo make test-kata -- kata1  # Run specific kata

# Code Quality
cargo clippy                # Run linter
cargo fmt                   # Format code
cargo make qa               # Run all quality checks
cargo make qa-fix           # Fix issues automatically

# Kata Management
cargo make list-katas           # Show all katas status
cargo make enable-kata -- kata2 # Enable specific kata
cargo make disable-kata -- kata2 # Disable specific kata
```

## 🏗️ Project Structure

```
rust_clean_code_katas/
├── src/
│   ├── main.rs                          # Entry point
│   ├── kata1_temperature_converter.rs   # Type safety & enums
│   ├── kata2_string_processor.rs        # Function extraction
│   ├── kata3_simple_calculator.rs       # Error handling
│   ├── kata4_user_validation.rs         # Newtype pattern
│   ├── kata5_shape_area_calculator.rs   # Traits & open/closed
│   ├── kata6_simple_logging_system.rs   # Dependency injection
│   ├── kata7_mini_todo_application.rs   # Clean architecture
│   └── kata8_simple_parser.rs           # Parser combinators
├── shell.nix                    # Nix development environment
├── rust-toolchain.toml         # Rust toolchain specification
├── verify-shell.sh             # Environment verification script
├── Cargo.toml                  # Project dependencies
├── clippy.toml                 # Linting configuration
├── rustfmt.toml               # Code formatting rules
└── CLAUDE.md                  # Development guidance
```

## 🎓 Learning Approach

1. **Start with Kata 1** - Each kata builds on concepts from previous ones
2. **Read the tests first** - Understand expected behavior before implementation
3. **Focus on one principle** - Each kata emphasizes specific clean code concepts
4. **Iterate and refactor** - Apply learned principles to improve your solutions
5. **Experiment** - Try different approaches and compare trade-offs

## 🤝 Contributing

Contributions are welcome! Whether you want to:

- Add new katas
- Improve existing exercises
- Fix bugs or typos
- Enhance documentation
- Share alternative solutions

Please feel free to open an issue or submit a pull request.

### Development Setup for Contributors

```bash
git clone https://github.com/yourusername/rust_clean_code_katas.git
cd rust_clean_code_katas
./verify-shell.sh
nix-shell
cargo make qa  # Run all quality checks
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by the clean code principles from Robert C. Martin
- Built with the amazing Rust programming language
- Environment managed with the Nix package manager

---

**Happy coding! 🦀✨**

*Master clean code principles through hands-on practice with Rust*