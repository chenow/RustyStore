{
  description = "A Nix flake for a Redis Clone in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit-env = {
      url = "github:chenow/nix-pre-commit";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      pre-commit-env,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        pre-commit-lib = pre-commit-env.lib.${system};
      in
      {
        devShells.default = pre-commit-lib.mkDevShell {
          extraPackages = with pkgs; [
            rustc
            cargo
            cargo-watch
            rustfmt
            clippy
            rust-analyzer
            codecrafters-cli
            redis
            cargo-nextest
          ];

        };

        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
