{pkgs ? import <nixpkgs> {}}: let
  mylist = with pkgs; [
    amdvlk
    cargo
    fish
    rustc
    vulkan-tools
    vulkan-loader
    vulkan-headers
  ];
in
  (pkgs.buildFHSEnv {
    name = "simple-x11-env";
    targetPkgs = pkgs: mylist;
    multiPkgs = pkgs: mylist;
    runScript = "fish";
  })
.env
