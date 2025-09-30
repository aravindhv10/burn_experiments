{pkgs ? import <nixpkgs> {}}: let
  mylist = with pkgs; [
    amdvlk
    cargo
    fish
    rustc
    vulkan-tools
    vulkan-loader
    vulkan-headers

    (pkgs.python312.withPackages (ps:
      with ps; [
        albumentations
        einops
        fastapi
        flask
        inotify-simple
        ipython
        jax
        lightning
        matplotlib
        multiprocess
        numpy
        onnx
        onnxruntime
        onnxscript
        opencv-python
        pillow
        python-multipart
        requests
        safetensors
        tensorboard
        tensorboardx
        timm
        torch
        torchmetrics
        torchvision
        transformers
        uvicorn
      ]))

  ];
in
  (pkgs.buildFHSEnv {
    name = "simple-x11-env";
    targetPkgs = pkgs: mylist;
    multiPkgs = pkgs: mylist;
    runScript = "fish";
  })
.env
