{
  lib,
  ocaml-ng,
}:

ocaml-ng.ocamlPackages_5_1.buildDunePackage {
  pname = "harulisp";
  version = "dev";
  minimalOCamlVersion = "5.1";
  src = lib.cleanSource ./.;
}
