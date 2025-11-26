#include "./all.hpp"

class infer_slave {
  c10::InferenceMode mode;
  torch::inductor::AOTIModelPackageLoader loader;
  torch::TensorOptions options;
  torch::Tensor input_tensor;
  std::vector<torch::Tensor> inputs;
  std::vector<torch::Tensor> outputs;
  torch::Tensor out_tensor;
  std::size_t bytes_to_copy;

public:
  inline void operator()(arg_input *in, unsigned int const batch_size,
                         arg_output *out) {

    inputs[0] = torch::from_blob(static_cast<void *>(in),
                                 {batch_size, SIZE_Y, SIZE_X, SIZE_C}, options);
    outputs = loader.run(inputs);
    out_tensor = outputs[0].contiguous().cpu();
    bytes_to_copy = batch_size * SIZE_O * sizeof(outtype);
    std::memcpy(out, out_tensor.data_ptr<outtype>(), bytes_to_copy);
  }

  infer_slave()
      : loader("model_output.pt2"),
        options(
            torch::TensorOptions().dtype(torch::kFloat32).device(torch::kCPU)) {
    inputs.resize(1);
  }

  ~infer_slave() {}
};

infer_slave slave;

extern "C" {

void mylibtorchinfer(arg_input *in, unsigned int const batch_size,
                     arg_output *out) {

  slave(in,batch_size,out);
}
}
