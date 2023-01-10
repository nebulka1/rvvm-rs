{ dev ? true
, pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/8c54d842d954.tar.gz") { }
}:
with pkgs;
mkShell {
  nativeBuildInputs = [
    pkgsCross.riscv64-embedded.stdenv.cc
    unixtools.xxd
  ] ++ lib.optionals dev [
    clang-tools
    nixpkgs-fmt
    qemu
  ];
}
