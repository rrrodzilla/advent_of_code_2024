{
  description = "Multi-environment project example";
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    dev-environments.url = "github:Govcraft/dev-environments";
  };
  outputs =
    inputs@{ flake-parts, nixpkgs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.dev-environments.flakeModules.rust
        inputs.dev-environments.flakeModules.node
        inputs.dev-environments.flakeModules.typst
      ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        {
          # Rust Development Environment Options
          # ----------------------------------
          # enable: boolean - Enable/disable the Rust environment
          # rustVersion: enum - Rust toolchain ("stable", "beta", "nightly") (default: "stable")
          # withTools: list of strings - Additional Rust tools to include (converted to cargo-*)
          # extraPackages: list of packages - Additional packages to include
          # ide.type: enum - IDE preference ("rust-rover", "vscode", "none") (default: "none")
          rust-dev = {
            enable = true;
            rustVersion = "stable";
            # Example configuration:
            # withTools =[]; # Will be prefixed with cargo-
            extraPackages = [ pkgs.linuxPackages.perf ];
            # ide.type = "none";
          };

          # Create the combined shell
          devShells.default = pkgs.mkShell {
            buildInputs = nixpkgs.lib.flatten (
              nixpkgs.lib.attrValues config.env-packages
              ++ [
                pkgs.perf-tools
                pkgs.ttyd
                pkgs.ffmpeg
                pkgs.gnuplot
                pkgs.vhs
                pkgs.kcachegrind
                pkgs.graphviz
                pkgs.valgrind
              ]
            );
            shellHook = nixpkgs.lib.concatStringsSep "\n" (nixpkgs.lib.attrValues config.env-hooks);
          };
        };
    };
}
