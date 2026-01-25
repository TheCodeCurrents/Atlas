{
  description = "Atlas - Rust and Typst project";

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
        
        # Use the stable Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # Build inputs for the Rust package
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
        ];

        buildInputs = with pkgs; [
          
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.darwin.apple_sdk.frameworks.Security
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
        ];

      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          
          nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [
            # Rust development tools
            cargo-watch
            cargo-edit
            cargo-audit
            cargo-outdated
            
            # C/C++
            gcc
            clang
            cmake
            gdb
            
            # Go
            go
            
            # Python
            python3
            
            # Zig
            zig
            
            # Documentation tools
            typst
            
            # Other needed tools
            git
          ]);

          # Environment variables
          RUST_BACKTRACE = "1";
          
          shellHook = ''
            echo "🦀 Rust development environment loaded"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
          '';
        };
      }
    );
}