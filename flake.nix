{
  description = "Rust broker-v2 dev environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            fenix.overlay
          ];
        };
        nativeBuildInputs = with pkgs; [
            (fenix.packages."${system}".stable.withComponents [
              "cargo"
              "rust-src"
              "rust-docs"
              "rustc"
            ])
            cargo-watch
            rust-analyzer
            pkg-config
            glib
            openssl
          ] ++ lib.optionals stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.CoreFoundation
          ];

      in {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs;
          RUST_SRC_PATH = "${fenix.packages.${system}.stable.rust-src}/bin/rust-lib/src";
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeBuildInputs;
          NIX_LDFLAGS="${pkgs.lib.optionalString pkgs.stdenv.isDarwin "\
            -F${pkgs.darwin.apple_sdk.frameworks.Security}/Library/Frameworks -framework Security \
            -F${pkgs.darwin.apple_sdk.frameworks.CoreFoundation}/Library/Frameworks -framework CoreFoundation"}";
          buildInputs = with pkgs; [
            (fenix.packages."${system}".stable.withComponents [ "clippy" "rustfmt"])
            just
          ];
        };
      }
    );
}
