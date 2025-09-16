{ pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  }
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    (rust-bin.stable.latest.default.override {
      extensions = [ "rust-src" "rustfmt" "clippy" ];
    })
    
    # Build tools
    pkg-config
    cargo-edit
    cargo-watch
    
    # Nushell for testing
    nushell
    
    # Development tools
    rust-analyzer
  ];

  shellHook = ''
    echo "Super Exit development environment"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo ""
    echo "Available commands:"
    echo "  cargo build         - Build the project"
    echo "  cargo run           - Run the project" 
    echo "  cargo test          - Run tests"
    echo "  cargo run -- --help - Show help"
    echo "  nu                  - Start Nushell for testing"
    echo ""
    echo "To test nested shells:"
    echo "  nu                  - Start first Nushell"
    echo "  nu                  - Start second Nushell (nested)"
    echo "  cargo run -- -v     - Run super-exit with verbose output"
  '';
}