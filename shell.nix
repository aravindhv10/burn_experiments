{pkgs ? import <nixpkgs> {}}: let
  mylist = with pkgs; [
    amdvlk
    cargo
    fish
    rustc
  ];
in
  (pkgs.buildFHSEnv {
    name = "simple-x11-env";

    targetPkgs = pkgs: mylist;

    multiPkgs = pkgs: mylist;

    runScript = "fish";
  })

.env
