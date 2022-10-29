{ pkgs ? import <nixpkgs> {}}:

let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  ruststable = (pkgs.latest.rustChannels.stable.default.override {
      extensions = [
        "rust-src"
        "rustfmt"
        "clippy"
      ];
    });
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    ruststable
  ];

  RUST_BACKTRACE = 1;
}

