{
  inputs = {
    nixpkgs.url = "flake:nixpkgs";
    flake-parts.url = "github:hercules-ci/flake-parts";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-hooks-nix = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nixpkgs-stable.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs@{ flake-parts, ... }:
    let
      lib = inputs.nixpkgs.lib // inputs.flake-parts.lib;
    in
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports =
        (lib.optionals (inputs.treefmt-nix ? flakeModule) [ inputs.treefmt-nix.flakeModule ])
        ++ (lib.optionals (inputs.pre-commit-hooks-nix ? flakeModule) [ inputs.pre-commit-hooks-nix.flakeModule ]);

      flake = {
        # Put your original flake attributes here.
      };
      #systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      systems = [
        # systems for which you want to build the `perSystem` attributes
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
      ];
      # perSystem = { config, self', inputs', pkgs, system, ... }: {
      perSystem = { config, pkgs, system, ... }:
        let
          craneLib = inputs.crane.lib.${system};
        in
        rec {
          # Per-system attributes can be defined here. The self' and inputs'
          # module parameters provide easy access to attributes of the same
          # system.

          packages.default = pkgs.callPackage ./package.nix { inherit craneLib; };

          devShells.default =
            craneLib.devShell {

              # Reference:
              # https://crane.dev/local_development.html

              inputsFrom = [ packages.default ];

              shellHook = ''
                # export DEBUG=1
                ${config.pre-commit.installationScript}
              '';
            };
        } // lib.optionalAttrs (inputs.pre-commit-hooks-nix ? flakeModule) {

          pre-commit = {
            check.enable = true;
            settings.hooks = {
              actionlint.enable = true;
              treefmt.enable = true;
            };
          };
        } // lib.optionalAttrs (inputs.treefmt-nix ? flakeModule) {
          treefmt.projectRootFile = ./flake.nix;
          treefmt.programs = {
            nixpkgs-fmt.enable = true;
            deadnix.enable = true;
            rustfmt.enable = true;
            statix.enable = true;
          };
        };
    };
}

