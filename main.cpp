#include <iostream>
#include <vector>

#include <torch/csrc/inductor/aoti_package/model_package_loader.h>
#include <torch/torch.h>

extern "C" {
torch::Tensor myfun(torch::Tensor in) {
  static c10::InferenceMode mode;
  static torch::inductor::AOTIModelPackageLoader loader("out.pt2");

  // std::vector<torch::Tensor> inputs = {torch::randn({4, 100}, at::kCPU)};
  std::vector<torch::Tensor> inputs = {in};
  std::vector<torch::Tensor> outputs = loader.run(inputs);
  // std::cout << "Result from the first inference:" << std::endl;
  std::cout << outputs[0] << std::endl;
  return outputs[0];
}
}

int main() {
  // std::vector<torch::Tensor> inputs = {torch::randn({4, 100}, at::kCPU)};
  std::cout <<  myfun(torch::randn({4, 100}, at::kCPU)) << std::endl;
  return 0;
}
