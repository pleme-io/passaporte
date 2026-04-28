{
  description = "passaporte — typed identity primitive for the saguão fleet";

  nixConfig = {
    allow-import-from-derivation = true;
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    crate2nix.url = "github:nix-community/crate2nix";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    substrate = {
      url = "github:pleme-io/substrate";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.fenix.follows = "fenix";
    };
    devenv = {
      url = "github:cachix/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crate2nix,
    flake-utils,
    substrate,
    devenv,
    ...
  }:
    # Library shape — substrate's rust-library builder.
    # Provides: packages (the library), devShells, apps (check-all,
    # bump, publish, release, regenerate).
    (import "${substrate}/lib/rust-library.nix" {
      inherit nixpkgs crate2nix flake-utils devenv;
    }) {
      crateName = "passaporte";
      src = self;
      repo = "pleme-io/passaporte";
    };
}
