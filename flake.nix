{
  description = "Super Exit - Recursively exit nested Nushell shells";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };

        super-exit = pkgs.rustPlatform.buildRustPackage {
          pname = "super-exit";
          version = "0.1.0";
          
          src = ./.;
          
          cargoHash = "sha256-0000000000000000000000000000000000000000000="; # Placeholder - will be updated by nix build
          
          nativeBuildInputs = with pkgs; [
            rustToolchain
            pkg-config
          ];
          
          buildInputs = with pkgs; [
            # Add any system dependencies here if needed
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          meta = with pkgs.lib; {
            description = "Recursively exit nested Nushell shells to close terminal";
            homepage = "https://github.com/gignsky/super-exit";
            license = with licenses; [ mit asl20 ];
            maintainers = [ ];
            platforms = platforms.all;
          };
        };

      in
      {
        packages = {
          default = super-exit;
          super-exit = super-exit;
        };

        apps = {
          default = flake-utils.lib.mkApp {
            drv = super-exit;
          };
          super-exit = flake-utils.lib.mkApp {
            drv = super-exit;
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            cargo-edit
            cargo-watch
            nushell
            
            # Development tools
            rust-analyzer
            clippy
            rustfmt
          ];

          shellHook = ''
            echo "Super Exit development environment"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo build       - Build the project"
            echo "  cargo run         - Run the project" 
            echo "  cargo test        - Run tests"
            echo "  cargo run -- -h   - Show help"
            echo "  nu                - Start Nushell for testing"
          '';
        };
      });
}