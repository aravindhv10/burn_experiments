#include <iostream>
#include <vector>

#include <torch/torch.h>
#include <torch/csrc/inductor/aoti_package/model_package_loader.h>

extern "C" {
int torchmain() {
    c10::InferenceMode mode;

    torch::inductor::AOTIModelPackageLoader loader("out.pt2");
    std::vector<torch::Tensor> inputs = {torch::randn({8, 100}, at::kCPU)};
    std::vector<torch::Tensor> outputs = loader.run(inputs);
    std::cout << "Result from the first inference:"<< std::endl;
    std::cout << outputs[0] << std::endl;

    return 0;
}
}
