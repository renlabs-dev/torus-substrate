{
  description = "Torus Substrate development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    pre-commit-hooks.url = "github:cachix/git-hooks.nix";
    pre-commit-hooks.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay, pre-commit-hooks, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        generalBuildInputs = with pkgs; [
          pkg-config
          openssl.dev
          rocksdb
          zstd.dev
        ];
        buildInputs =
          if pkgs.stdenv.isLinux then
            generalBuildInputs ++ [ pkgs.jemalloc pkgs.pkgsi686Linux.glibc ]
          else
            generalBuildInputs
            ++ [ pkgs.darwin.apple_sdk.frameworks.SystemConfiguration ];
        nativeBuildInputs = with pkgs; [
          git
          llvmPackages_17.stdenv
          llvmPackages_17.libcxx
          llvmPackages_17.libcxxStdenv
          llvmPackages_17.clang-unwrapped
          rust
          protobuf
          sccache
        ];

        shellPkgs = [
          pkgs.bashInteractive
          # Run project-specific commands
          pkgs.just
          # Python
          pkgs.python310
          # Subxt CLI for metadata handling
          pkgs.subxt
          pkgs.cargo-nextest
          # # Code coverage tool
          # pkgs.cargo-llvm-cov # marked as broken
        ];
      in
      {
        checks = pkgs.mkShell {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
               rustfmt.enable = true;

               push = {
                 enable = true;
                 name = "Tests & Stuff";
                 entry = "just test";
                 pass_filenames = false;
                 stages = ["pre-push"];
               };
            };
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = buildInputs;
          nativeBuildInputs = nativeBuildInputs;
          packages = shellPkgs;

          shellHook = ''
            ${self.checks.${system}.pre-commit-check.shellHook}

            # Correct paths to build RocksDB and WASM-OPT.
            # Don't touch it unless you know what you are doing.
            export C_INCLUDE_PATH="${pkgs.llvmPackages_17.clang-unwrapped.lib}/lib/clang/17/include"
            export CPLUS_INCLUDE_PATH="${pkgs.llvmPackages_17.libcxx.dev}/include/c++/v1:${pkgs.llvmPackages_17.clang-unwrapped.lib}/lib/clang/17/include"

            # Ensure clang-17 is prioritized in PATH. Oxalica's Rust Overlay
            # also ships clang-19 but current polkadot dependencies require 17.
            export PATH="${pkgs.llvmPackages_17.clang-unwrapped}/bin:$PATH"
            echo "Using clang version: $(clang --version | head -n1)"
          '';

          env = {
            LIBCLANG_PATH = "${pkgs.llvmPackages_17.clang-unwrapped.lib}/lib";
            ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
            ZSTD_SYS_USE_PKG_CONFIG = "true";
            OPENSSL_NO_VENDOR = "1";
            OPENSSL_DIR = "${pkgs.openssl.dev}";
            OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
            OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
            RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
            RUST_BACKTRACE = "1";
          } // nixpkgs.lib.optionalAttrs pkgs.stdenv.isLinux {
            JEMALLOC_OVERRIDE = "${pkgs.jemalloc}/lib/libjemalloc.so";
          };
        };
      });
}
