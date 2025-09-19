# Simplified Nix development environment for Rust clean code katas
# Focused on learning and practicing clean code principles

let
  # Stable nixpkgs for Rust development environment
  rustPkgs = import (fetchTarball "https://github.com/nixos/nixpkgs/tarball/nixos-25.05") {
    overlays = [
      # Import rust overlay for specific version control
      (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  };

  # Unstable nixpkgs for additional development utilities
  utilityPkgs = import (fetchTarball "https://github.com/nixos/nixpkgs/tarball/nixos-unstable") {
    config = {
      allowUnfree = true;  # Required for some utilities
    };
  };

  # Import rust toolchain from rust-toolchain.toml
  rust = (rustPkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);

in
rustPkgs.mkShell {
  nativeBuildInputs = [
    # Rust toolchain
    rust

    # Essential development tools
    rustPkgs.cargo-make        # Required prerequisite (mentioned in CLAUDE.md)
    rustPkgs.pkg-config        # System dependencies

    # Additional development utilities from unstable
    utilityPkgs.claude-code    # AI assistant CLI
    utilityPkgs.gemini-cli     # Google Gemini CLI
    utilityPkgs.kiro           # Development utility
    utilityPkgs.zed-editor     # Code editor
  ];

  # Environment variables for Rust development
  RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
  LIBCLANG_PATH = "${rustPkgs.libclang.lib}/lib";

  # Allow unfree packages in the shell
  NIXPKGS_ALLOW_UNFREE = "1";
}