# super-exit

Runs Exit Recursively in Nushell in an effort to close the terminal with nested shells inside one command

## Overview

`super-exit` is a Rust utility designed primarily for Nushell environments that can recursively exit nested shells to close the terminal completely. It detects nested shell processes and attempts to exit them all in sequence.

## Features

- **Nushell-first design**: Optimized for Nushell environments with backwards compatibility
- **Nested shell detection**: Automatically detects and counts nested shell levels
- **Verbose mode**: Detailed output showing detected processes and actions
- **Dry-run mode**: Preview what would be done without actually exiting
- **Cross-platform**: Works on Linux, macOS, and Windows

## Installation

### Using Nix Flakes (Recommended)

```bash
# Run directly
nix run github:gignsky/super-exit

# Install to profile
nix profile install github:gignsky/super-exit
```

### Using Nix (Traditional)

```bash
# Enter development environment
nix-shell

# Build with nix-build (when added to nixpkgs)
nix-build -A super-exit
```

### Using Cargo

```bash
# Install from source
cargo install --path .

# Or run directly
cargo run
```

## Usage

```bash
# Basic usage - exit all nested shells
super-exit

# See what would be done without exiting
super-exit --dry-run

# Verbose output showing detected processes
super-exit --verbose

# Count nested shells without exiting
super-exit --count

# Combine options
super-exit --dry-run --verbose
```

## Examples

### In Nushell

```nushell
# Start nested Nushell sessions
nu
nu
nu

# Check how many nested shells
super-exit --count
# Output: Nested shells detected: 3

# Exit all at once
super-exit
```

### In other shells

```bash
# Works in bash, zsh, fish, etc.
bash
bash
super-exit --verbose
# Will detect and exit nested bash shells
```

## Command Line Options

- `-v, --verbose`: Enable verbose output showing detected processes
- `-n, --dry-run`: Show what would be done without actually exiting  
- `-c, --count`: Just count and display nested shells without exiting
- `-h, --help`: Show help message
- `-V, --version`: Show version information

## How it Works

1. **Detection**: Scans parent processes to identify shell types (Nushell, bash, zsh, etc.)
2. **Counting**: Counts the number of nested shell levels
3. **Strategy**: 
   - For Nushell: Uses `nu -c "exit"` commands recursively
   - For other shells: Uses standard exit mechanisms
4. **Execution**: Exits shells in sequence from innermost to outermost

## Development

### Prerequisites

- Rust 1.70+ (for building)
- Nix (optional, for Nix-based development)
- Nushell (optional, for testing Nushell-specific features)

### Building

```bash
# Standard Rust build
cargo build

# Nix development environment
nix develop  # or nix-shell

# Run tests
cargo test

# Check code
cargo clippy
```

### Testing

```bash
# Test in a nested environment
nu
nu
cargo run -- --count --verbose

# Test dry-run functionality  
cargo run -- --dry-run --verbose
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and clippy
5. Submit a pull request

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Future Plans

- Add to nixpkgs for easier installation
- Support for more shell types
- Configuration file support
- Plugin system for custom exit strategies
