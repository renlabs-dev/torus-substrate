{
  description = "Torus Substrate development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/24.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    pre-commit-hooks.url = "github:cachix/git-hooks.nix";
  };

  outputs = { self, nixpkgs, rust-overlay, pre-commit-hooks, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        generalBuildInputs = with pkgs; [
          pkg-config
          openssl.dev
          rocksdb
          zstd.dev
        ];
        buildInputs =
          if pkgs.stdenv.isLinux
          then generalBuildInputs ++ [ pkgs.jemalloc pkgs.pkgsi686Linux.glibc ]
          else generalBuildInputs ++ [ pkgs.darwin.apple_sdk.frameworks.SystemConfiguration ];
        nativeBuildInputs = with pkgs; [ git rust clang protobuf sccache ];

        shellPkgs = [
          pkgs.bashInteractive
          # Run project-specific commands
          pkgs.just
          # Run Github actions locally
          pkgs.act
        ];
      in
      {
        checks = pkgs.mkShell {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              rustfmt.enable = true;
            };
          };
        };

        devShells.default = pkgs.mkShell {
          inherit (self.checks.${system}.pre-commit-check) shellHook;
          buildInputs = buildInputs;
          nativeBuildInputs = nativeBuildInputs;
          packages = shellPkgs;

          env = {
            LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
            ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
            ZSTD_SYS_USE_PKG_CONFIG = "true";
            OPENSSL_NO_VENDOR = "1";
            OPENSSL_DIR = "${pkgs.openssl.dev}";
            OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
            OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
            RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
            RUST_BACKTRACE = "1";
          } // nixpkgs.lib.optionalAttrs pkgs.stdenv.isLinux { JEMALLOC_OVERRIDE = "${pkgs.jemalloc}/lib/libjemalloc.so"; };
        };
      }
    );
}
