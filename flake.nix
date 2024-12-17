# SPDX-FileCopyrightText: 2021 Serokell <https://serokell.io/>
#
# SPDX-License-Identifier: CC0-1.0

{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
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
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };

        # Darwin-specific dependencies
        darwinDeps = pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.SystemConfiguration
          libiconv
        ]);

        nativeBuildInputs = with pkgs; [
          pkg-config
          rustToolchain
          sccache
        ] ++ pkgs.lib.optionals (!pkgs.stdenv.isDarwin) [ pkgs.lld ];

        buildInputs = with pkgs; [
          openssl
          libgit2
          zlib
          act
        ] ++ darwinDeps;

        # Platform-specific Cargo config
        cargoConfig = if pkgs.stdenv.isDarwin then ''
          [source.crates-io]
          registry = "https://github.com/rust-lang/crates.io-index"

          [target.${pkgs.stdenv.hostPlatform.config}]
          linker = "cc"

          [build]
          rustc-wrapper = "sccache"
        '' else ''
          [source.crates-io]
          registry = "https://github.com/rust-lang/crates.io-index"

          [target.${pkgs.stdenv.hostPlatform.config}]
          linker = "cc"
          rustflags = ["-C", "link-arg=-fuse-ld=lld"]

          [build]
          rustc-wrapper = "sccache"
        '';

      in {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;
          
          shellHook = ''
            # Platform-specific library path setup
            export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.libgit2}/lib/pkgconfig:$PKG_CONFIG_PATH"
            
            # OpenSSL configuration
            export OPENSSL_DIR="${pkgs.openssl.dev}"
            export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
            export OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include"
            
            # LibGit2 configuration
            export LIBGIT2_SYS_USE_PKG_CONFIG=1
            export LIBGIT2_PATH="${pkgs.libgit2}"
            
            # Rust specific env vars
            export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"
            export RUST_BACKTRACE=1
            
            # macOS specific configuration
            ${if pkgs.stdenv.isDarwin then ''
              export OPENSSL_STATIC=0
              export OPENSSL_NO_VENDOR=1
            '' else ""}
            
            # Setup Cargo config if it doesn't exist
            if [ ! -f .cargo/config.toml ]; then
              mkdir -p .cargo
              cat > .cargo/config.toml << EOF
            ${cargoConfig}
            EOF
            fi

            # Initialize sccache
            export SCCACHE_DIR="$PWD/.sccache"
            export RUSTC_WRAPPER="sccache"
            mkdir -p "$SCCACHE_DIR"
          '';

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };

        # For backwards compatibility
        devShell = self.devShells.${system}.default;
      });
}