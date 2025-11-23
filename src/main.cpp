#include "./all.hpp"

extern "C" {

void do_infer(arg_input *in, unsigned int batch_size, arg_output *out) {

  static c10::InferenceMode mode;
  static torch::inductor::AOTIModelPackageLoader loader("model_output.pt2");

  auto options = torch::TensorOptions().dtype(torch::kFloat32).device(torch::kCPU);

  torch::Tensor input_tensor = torch::from_blob(
      static_cast<void*>(in), 
      {batch_size, SIZE_Y, SIZE_X, SIZE_C}, 
      options
  );

  std::vector<torch::Tensor> inputs = {input_tensor};

  // inputs.reserve(1);
  // inputs.push_back(input_tensor);

  // std::vector<torch::Tensor> inputs = {
  //     torch::zeros({batch_size, SIZE_Y, SIZE_X, SIZE_C}, at::kCPU)};

  // for (int B = 0; B < batch_size; ++B) {
  //   for (int Y = 0; Y < SIZE_Y; ++Y) {
  //     for (int X = 0; X < SIZE_X; ++X) {
  //       for (int C = 0; C < SIZE_C; ++C) {
  //         inputs[0][B][Y][X][C] = in[B].val[Y][X][C];
  //       }
  //     }
  //   }
  // }

  std::vector<torch::Tensor> outputs = loader.run(inputs);

  for (int B = 0; B < batch_size; ++B) {
    for (int O = 0; O < SIZE_O; ++O) {
      out[B].val[O] = outputs[0][B][O].item<outtype>();
    }
  }
}
}
