{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell rec {
    buildInputs = with pkgs; [
      rustup
      qemu
    ];
    # CARGO_BUILD_TARGET = "x86_64-kernel.json";
    # RUSTC_VERSION = pkgs.lib.readFile ./rust-toolchain;
  }