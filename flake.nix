{
  description = "A development shell for rust";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    crane,
    treefmt-nix,
    ...
  }: let
    # Define systems
    systems = [
      "x86_64-linux"
      "aarch64-darwin"
      "x86_64-darwin"
    ];

    # Helper function to generate per-system attributes
    forAllSystems = f: nixpkgs.lib.genAttrs systems f;
  in {
    # Optional: Define packages if using crane to build (uncomment to use)
    # packages = forAllSystems (system: let
    #   pkgs = import nixpkgs {
    #     inherit system;
    #     overlays = [rust-overlay.overlays.default];
    #   };
    #   craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default);
    # in {
    #   default = craneLib.buildPackage {
    #     src = craneLib.cleanCargoSource ./.;
    #     strictDeps = true;
    #   };
    # });

    # Define devShells for all systems
    devShells = forAllSystems (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };
      # Optional: Initialize crane for building packages
      # craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default);
      # Optional: Example crane package build (uncomment to use)
      # my-crate = craneLib.buildPackage {
      #   src = craneLib.cleanCargoSource ./.;
      #   strictDeps = true;
      # };
    in {
      default = pkgs.mkShell {
        name = "dev";
        # Available packages on https://search.nixos.org/packages
        buildInputs = with pkgs; [
          alejandra # Nix
          nixd
          statix
          deadnix
          just
          rust-bin.stable.latest.default
          rust-analyzer
          clippy
        ];
        shellHook = ''
          echo "Welcome to the rust devshell!"
        '';
      };
    });

    formatter = forAllSystems (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };
      treefmtModule = {
        projectRootFile = "flake.nix";
        programs = {
          alejandra.enable = true; # Nix formatter
          rustfmt.enable = true; # Rust formatter
        };
      };
    in
      treefmt-nix.lib.mkWrapper pkgs treefmtModule);
  };
}
