#include <iostream>
#include <vector>

#include <torch/csrc/inductor/aoti_package/model_package_loader.h>
#include <torch/torch.h>

extern "C" {

struct arg_input {
  float val[100];
};

struct arg_output {
  float val[4];
};

arg_output torchmain(arg_input in) {
  static c10::InferenceMode mode;
  static torch::inductor::AOTIModelPackageLoader loader("out.pt2");

  std::vector<torch::Tensor> inputs = {torch::zeros({1, 100}, at::kCPU)};
  for (int i = 0; i < 100; i++) {
    inputs[0][0][i] = in.val[i];
  }

  std::vector<torch::Tensor> outputs = loader.run(inputs);

  arg_output out;
  for (int i = 0; i < 4; i++) {
    out.val[i] = outputs[0][0][i].item<float>();
  }
  return out;
}
}
