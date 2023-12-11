{
  description = "SUDOKU solver written in rust";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-PjvuouwTsYfNKW5Vi5Ye7y+lL7SsWGBxCtBOOm2z14c=";
        };
        pkgs = import nixpkgs { inherit system; };
        rustPlatform = pkgs.makeRustPlatform
          {
            rustc = toolchain;
            cargo = toolchain;
          };
      in
      {
        # https://nixos.org/manual/nix/stable/command-ref/new-cli/nix3-fmt
        formatter = pkgs.nixpkgs-fmt;

        devShells.default = pkgs.mkShell {
          packages = [ toolchain ];
        };

        packages.default =
          rustPlatform.buildRustPackage {
            pname = "sudoku";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
      }
    );
}
