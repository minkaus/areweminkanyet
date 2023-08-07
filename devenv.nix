{ inputs, ... }:
  let
    rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
    pkgs = import inputs.nixpkgs { overlays = [rust_overlay]; };

  in {
    packages = with pkgs; [git wasm-pack yarn];

    languages = {
      nix.enable = true;
      rust = with pkgs.rust-bin.stable.latest; {
        enable = true;
        toolchain = {
          inherit cargo rust-src rustfmt clippy;
          rustc = minimal.override { targets = [ "wasm32-unknown-unknown" ]; };
        };
      };
    };

    pre-commit = {
      hooks.clippy.enable = true;
      hooks.rustfmt.enable = true;
    };
  }
