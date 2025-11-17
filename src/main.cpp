#include <torch/csrc/inductor/aoti_package/model_package_loader.h>
#include <torch/torch.h>
#include <vector>

extern "C" {

unsigned long constexpr INPUT_SIZE = 100;
unsigned long constexpr OUTPUT_SIZE = 4;

struct arg_input {
  float val[INPUT_SIZE];
};

struct arg_output {
  float val[OUTPUT_SIZE];
};

void do_infer(arg_input const *in, unsigned int batch_size, arg_output *out) {
  static c10::InferenceMode mode;
  static torch::inductor::AOTIModelPackageLoader loader("out.pt2");

  std::vector<torch::Tensor> inputs = {
      torch::zeros({batch_size, INPUT_SIZE}, at::kCPU)};
  for (int j = 0; j < batch_size; ++j) {
    for (int i = 0; i < INPUT_SIZE; ++i) {
      inputs[0][j][i] = in[j].val[i];
    }
  }

  std::vector<torch::Tensor> outputs = loader.run(inputs);

  for (int j = 0; j < batch_size; ++j) {
    for (int i = 0; i < OUTPUT_SIZE; i++) {
      out[j].val[i] = outputs[0][j][i].item<float>();
    }
  }
}
}
