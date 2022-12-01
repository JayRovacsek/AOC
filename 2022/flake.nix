{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    nixpkgs.follows = "cargo2nix/nixpkgs";

    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs = {
        nixpkgs.follows = "cargo2nix/nixpkgs";
        flake-utils.follows = "flake-utils";
        flake-compat.follows = "flake-compat";
      };
    };
  };

  outputs = inputs:
    with inputs;
    flake-utils.lib.eachSystem [
      "aarch64-linux"
      "aarch64-darwin"
      "x86_64-darwin"
      "x86_64-linux"
    ] (system:
      let

        inherit (inputs.nixpkgs.lib.systems) examples;
        # There is no reason for the below, beyond 'I can'
        # Wanted to toy with this construct to understand if it could
        # be a better option than the cross option (which in truth
        # is damn awesome)
        cross-targets = {
          x86_64_darwin = examples.x86_64-darwin;
          aarch64_darwin = examples.aarch64-darwin;
          x86_64_windows = examples.mingwW64;
          x86_64_linux = examples.gnu64;
          aarch64_linux = examples.aarch64-multiplatform;
        };

        cross-map = {
          x86_64_darwin = "x86_64-apple-darwin";
          aarch64_darwin = "aarch64-apple-darwin";
          x86_64_windows = "x86_64-pc-windows-gnu";
          x86_64_linux = "x86_64-unknown-linux-gnu";
          aarch64_linux = "aarch64-unknown-linux-gnu";
        };

        overlays = [ cargo2nix.overlays.default ];

        pkgs = import nixpkgs { inherit system overlays; };

        cross-pkgs = builtins.mapAttrs (name: value:
          import nixpkgs {
            inherit system overlays;
            crossSystem = value;
          }) cross-targets;

        packageFun = import ./Cargo.nix;
        rustVersion = inputs.nixpkgs.legacyPackages.${system}.rustc.version;

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          inherit packageFun rustVersion;
          packageOverrides = pkgs: pkgs.rustBuilder.overrides.all;
        };

        cross-rustPkgs = builtins.mapAttrs (name: value:
          cross-pkgs.${name}.rustBuilder.makePackageSet {
            inherit packageFun rustVersion;
            target = cross-map.${name};
          }) cross-targets;

        aoc_2022 = (rustPkgs.workspace.aoc_2022 { }).bin;

        cross-compile =
          builtins.mapAttrs (name: value: ((value.workspace.aoc_2022 { }).bin))
          cross-rustPkgs;

        checks = {
          pre-commit-check = self.inputs.pre-commit-hooks.lib.${system}.run
            (import ./pre-commit-checks.nix { inherit self pkgs system; });
        };

        stableDeps = with pkgs; [ nixfmt ];

        # TODO: resolve merge of the below with cargo2nix dev shell (overlay)
        devShell = pkgs.mkShell {
          name = "rust-dev-shell";
          packages = stableDeps;
          inherit (self.checks.${system}.pre-commit-check) shellHook;
        };

        aoc-nix = { one = import ./nix/one { inherit pkgs; }; };

      in {
        inherit checks;
        # inherit checks devShell;
        devShell = inputs.cargo2nix.outputs.devShells.${system}.default;
        packages = {
          inherit aoc_2022 aoc-nix cross-compile;
          default = aoc_2022;
        };
      });
}
