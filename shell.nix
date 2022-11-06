{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell rec {
    buildInputs = with pkgs; [
      rustup
      grub2
      xorriso
      qemu
      bochs
    ];
    # CARGO_BUILD_TARGET = "x86_64-kernel.json";
    # RUSTC_VERSION = pkgs.lib.readFile ./rust-toolchain;
  }