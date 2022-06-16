{
  description = "Rust broker-v2 dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
    deploy-rs.url = "github:serokell/deploy-rs";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    naersk,
    flake-utils,
    deploy-rs,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            fenix.overlay
            deploy-rs.overlay
          ];
        };
        nativeBuildInputs =
          [
            (fenix.packages."${system}".stable.withComponents [
              "cargo"
              "rust-src"
              "rust-docs"
              "rustc"
            ])
            pkgs.rust-analyzer
            pkgs.pkg-config
            pkgs.glib
            pkgs.openssl
          ]
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.CoreFoundation
          ];
      in rec {
        checks = deploy-rs.lib."${system}".deployChecks {
          nodes = pkgs.lib.filterAttrs (name: cfg: cfg.profiles.system.path.system == system) self.deploy.nodes;
        };

        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs;
          RUST_SRC_PATH = "${fenix.packages.${system}.stable.rust-src}/bin/rust-lib/src";
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeBuildInputs;
          NIX_LDFLAGS = "${pkgs.lib.optionalString pkgs.stdenv.isDarwin "\
            -F${pkgs.darwin.apple_sdk.frameworks.Security}/Library/Frameworks -framework Security \
            -F${pkgs.darwin.apple_sdk.frameworks.CoreFoundation}/Library/Frameworks -framework CoreFoundation"}";
          buildInputs = with pkgs; [
            (fenix.packages."${system}".stable.withComponents ["clippy" "rustfmt"])
            pkgs.just
            deploy-rs.packages."${system}".deploy-rs
          ];
        };
      }
    )
    // {
      nixosConfigurations = {
        "keys.walletconnect.com" = nixpkgs.lib.nixosSystem {
          system = "x86_64-linux";
          modules = [
            ({
              pkgs,
              config,
              ...
            }: {
              nix = {
                extraOptions = ''
                  experimental-features = nix-command flakes
                  keep-outputs = true
                  keep-derivations = true
                '';
                trustedUsers = ["root"];
              };
              networking.hostName = "keys-walletconnect-com";
            })
            ./hosts/keys.walletconnect.com
          ];
        };
      };

      deploy.nodes = {
        "keys.walletconnect.com" = {
          hostname = "159.65.123.131";
          sshUser = "root";
          fastConnection = true;
          profiles.system = {
            user = "root";
            path =
              deploy-rs.lib.x86_64-linux.activate.nixos
              self.nixosConfigurations."keys.walletconnect.com";
          };
        };
      };
    };
}
