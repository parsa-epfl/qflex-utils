let
  pkgs = import <nixpkgs> {};
in
  pkgs.mkShell {
    packages = [
      pkgs.rustc
      pkgs.rustfmt
      pkgs.rustup
      pkgs.cargo,
      pkgs.cmake,
      pkgs.lldb
    ];

    env = {
      # ENV = "value"
    };

    shellHook = ''
        fish -li
    '';
  }