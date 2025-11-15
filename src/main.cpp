#include <iostream>
#include <vector>

#include <torch/csrc/inductor/aoti_package/model_package_loader.h>
#include <torch/torch.h>

extern "C" {

unsigned long constexpr INPUT_SIZE = 100;
unsigned long constexpr OUTPUT_SIZE = 4;

struct arg_input {
  float val[INPUT_SIZE];
};

struct arg_output {
  float val[OUTPUT_SIZE];
};

arg_output do_infer(arg_input const in) {
  static c10::InferenceMode mode;
  static torch::inductor::AOTIModelPackageLoader loader("out.pt2");

  std::vector<torch::Tensor> inputs = {torch::zeros({1, INPUT_SIZE}, at::kCPU)};
  for (int i = 0; i < INPUT_SIZE; i++) {
    inputs[0][0][i] = in.val[i];
  }

  std::vector<torch::Tensor> outputs = loader.run(inputs);

  arg_output out;
  for (int i = 0; i < OUTPUT_SIZE; i++) {
    out.val[i] = outputs[0][0][i].item<float>();
  }
  return out;
}
}
