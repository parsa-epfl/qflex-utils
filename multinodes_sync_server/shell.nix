let
  pkgs = import <nixpkgs> {};
in
  pkgs.mkShell {
    packages = [
      pkgs.rustc
      pkgs.rustfmt
      pkgs.rustup
      pkgs.cargo
    ];

    env = {
      # ENV = "value"
    };

    shellHook = ''
        fish -li
    '';
  }