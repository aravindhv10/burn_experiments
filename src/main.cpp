#include "./all.hpp"

extern "C" {

unsigned int constexpr IMAGE_RESILUTION = 448;
unsigned int constexpr NUM_CHANNELS = 3;
unsigned int constexpr NUM_CLASSES = 3;

unsigned int constexpr SIZE_Y = IMAGE_RESILUTION;
unsigned int constexpr SIZE_X = IMAGE_RESILUTION;
unsigned int constexpr SIZE_C = NUM_CHANNELS;

unsigned int constexpr SIZE_O = NUM_CLASSES;

typedef float outtype;

struct arg_input {
  outtype val[SIZE_Y][SIZE_X][SIZE_C];
};

struct arg_output {
  outtype val[SIZE_O];
};

void do_infer(arg_input const *in, unsigned int batch_size, arg_output *out) {

  static c10::InferenceMode mode;
  static torch::inductor::AOTIModelPackageLoader loader("model_output.pt2");

  std::vector<torch::Tensor> inputs = {
      torch::zeros({batch_size, SIZE_Y, SIZE_X, SIZE_C}, at::kCPU)};

  for (int B = 0; B < batch_size; ++B) {
    for (int Y = 0; Y < SIZE_Y; ++Y) {
      for (int X = 0; X < SIZE_X; ++X) {
        for (int C = 0; C < SIZE_C; ++C) {
          inputs[0][B][Y][X][C] = in[B].val[Y][X][C];
        }
      }
    }
  }

  std::vector<torch::Tensor> outputs = loader.run(inputs);

  for (int B = 0; B < batch_size; ++B) {
    for (int O = 0; O < SIZE_O; ++O) {
      out[B].val[O] = outputs[0][B][O].item<outtype>();
    }
  }
}
}
