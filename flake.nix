{
  description = "Time Tracker";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-25.11";
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
        inputsRun = with pkgs; [
          openssl
          sqlite
        ];
        inputsCompile = with pkgs; [
          toolchain
          pkg-config
          gcc
          makeWrapper
        ] ++ inputsRun;
        inputsDev = with pkgs; [
          cargo-expand
          cargo-public-api
          rust-analyzer-nightly
          diesel-cli
        ] ++ inputsCompile;
        LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath inputsRun;
      in with pkgs; rec
      {
        devShells.default = mkShell {
          inherit LD_LIBRARY_PATH;
          
          nativeBuildInputs = inputsDev;

          DATABASE_URL = "/home/inferno214221/projects/owned/invoice-generator/main.sqlite";
        };

        packages.default = (naersk.buildPackage {          
          src = ./.;
          
          buildInputs = inputsRun;
          nativeBuildInputs = inputsCompile;
        }).overrideAttrs {
          postFixup = ''
            wrapProgram $out/bin/time-tracker --set LD_LIBRARY_PATH ${LD_LIBRARY_PATH}
          '';
        };

        apps.default = {
          type = "app";
          program = "${packages.default}/bin/time-tracker";
        };
      }
    );
}
