{ pkgs ? import <nixpkgs> { } }: with pkgs;
mkShell rec {
  buildInputs = [
    lzo
  ];
  LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
}
