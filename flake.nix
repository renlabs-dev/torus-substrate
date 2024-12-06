{
  description = "Torus Substrate development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/24.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        generalBuildInputs = with pkgs; [
          bashInteractive
          glibc_multi.dev
          openssl.dev
          pkg-config
          rocksdb
          zstd.dev
        ];
        buildInputs = if pkgs.stdenv.isLinux
          then generalBuildInputs ++ [ pkgs.jemalloc pkgs.pkgsi686Linux.glibc ]
          else generalBuildInputs;
        nativeBuildInputs = with pkgs; [ git rust clang protobuf sccache ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = buildInputs;
          nativeBuildInputs = nativeBuildInputs;

          env = {
            LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
            ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
            ZSTD_SYS_USE_PKG_CONFIG = "true";
            OPENSSL_DIR = "${pkgs.openssl.dev}";
            OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
            OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
            RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          } // nixpkgs.lib.optionalAttrs pkgs.stdenv.isLinux { JEMALLOC_OVERRIDE = "${pkgs.jemalloc}/lib/libjemalloc.so"; };
        };
      }
    );
}
