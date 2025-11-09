{
  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.xz";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      perSystem = {
        lib,
        pkgs,
        system,
        ...
      }: {
        _module.args.pkgs = with inputs;
          import nixpkgs {
            inherit system;
            overlays = [(import rust-overlay)];
          };

        packages.default = pkgs.stdenv.mkDerivation {
          pname = "libobs-rs";
          version = "3.0.0";

          src = ./.;

          buildInputs = with pkgs; [
            openssl
            pkg-config
            rust-bin.stable.latest.default
          ];
        };
      };

      systems = inputs.nixpkgs.lib.systems.flakeExposed;
    };
}
