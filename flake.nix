{
  description = "A Nix flake for a Redis Clone in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-env = {
      url = "github:chenow/nix-pre-commit";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      pre-commit-env,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
          (self: super: {
            rust-toolchain = self.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          })
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        pre-commit-lib = pre-commit-env.lib.${system};
      in
      {
        devShells.default = pre-commit-lib.mkDevShell {
          extraPackages = with pkgs; [
            rust-toolchain
            cargo-watch
            cargo-nextest
            redis
          ];
        };

        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
