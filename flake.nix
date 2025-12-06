{
  description = "Invoice Generator";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    naersk-pkg = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, naersk-pkg, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ fenix.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-P39FCgpfDT04989+ZTNEdM/k/AE869JKSB4qjatYTSs=";
        };
        naersk = pkgs.callPackage naersk-pkg {
          cargo = toolchain;
          rustc = toolchain;
        };
        buildInputs = with pkgs; [
          diesel-cli
        ];
        nativeBuildInputs = with pkgs; [
          toolchain
          pkg-config
          gcc
          cargo-expand
          cargo-public-api
          rust-analyzer-nightly
          openssl
          sqlite
        ] ++ buildInputs;
      in with pkgs; rec
      {
        devShells.default = mkShell {
          inherit nativeBuildInputs;

          LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath [
            pkgs.openssl
            pkgs.sqlite
          ];

          DATABASE_URL = "/home/inferno214221/projects/owned/invoice-generator/main.sqlite";
        };
      }
    );
}
