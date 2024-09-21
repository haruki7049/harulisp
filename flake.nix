{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-parts.url = "github:hercules-ci/flake-parts";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      imports = [ inputs.treefmt-nix.flakeModule ];
      perSystem =
        { pkgs, ... }:
        let
          ocamlPackages = pkgs.ocaml-ng.ocamlPackages_5_1;
          harulisp = pkgs.callPackage ./. { };
        in
        {
          treefmt = {
            projectRootFile = "flake.nix";
            programs.nixfmt-rfc-style.enable = true;
          };

          checks = {
            inherit harulisp;
          };

          packages = {
            inherit harulisp;
            default = harulisp;
          };

          devShells.default = pkgs.mkShell {
            nativeBuildInputs = [
              ocamlPackages.ocaml
              ocamlPackages.ocaml-lsp
              ocamlPackages.dune_3
            ];

            buildInputs = with ocamlPackages; [ ];

            shellHook = ''
              export PS1="\n[nix-shell:\w]$ "
            '';
          };
        };
    };
}
