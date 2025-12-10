#include "./main.hpp"

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

    torch::Tensor cpu_tensor = torch::from_blob(static_cast<void *>(in), {batch_size, SIZE_Y, SIZE_X, SIZE_C}, torch::kCPU);

    inputs[0] = cpu_tensor.to(options);
    outputs = loader.run(inputs);
    out_tensor = outputs[0].contiguous().cpu();
    bytes_to_copy = batch_size * SIZE_O * sizeof(outtype);
    std::memcpy(out, out_tensor.data_ptr<outtype>(), bytes_to_copy);
  }

  infer_slave()
      : loader("/model.pt2"),
        options(
            torch::TensorOptions().dtype(torch::kFloat32).device(torch::kCUDA)) {
    inputs.resize(1);
  }

  ~infer_slave() {}
};

infer_slave slave;

inline cv::Mat process_image_data(unsigned char *binary_data, int data_size) {
  cv::Mat decoded_img = cv::imdecode(cv::Mat(1, data_size, CV_8UC1, binary_data), cv::IMREAD_COLOR);
  if (decoded_img.empty()) {return cv::Mat::zeros(SIZE_Y, SIZE_X, CV_8UC1);}

  int const height = decoded_img.rows;
  int const width = decoded_img.cols;
  int target_height = IMAGE_RESOLUTION;
  int target_width = IMAGE_RESOLUTION;
  int x_start = 0;
  int y_start = 0;
  float ar = 1;

  if (height < width) {
    ar = float(width) / float(height) ;
    target_width = int(float(IMAGE_RESOLUTION) * ar);
    x_start = static_cast<unsigned int>(target_width - IMAGE_RESOLUTION) >> 1;
  } else {
    ar = float(height) / float(width) ;
    target_height = int(float(IMAGE_RESOLUTION) * ar);
    y_start = static_cast<unsigned int>(target_height - IMAGE_RESOLUTION) >> 1;
  }

  cv::Mat resized_img;
  if ((height > IMAGE_RESOLUTION) && (width > IMAGE_RESOLUTION)) {
    cv::resize(decoded_img, resized_img, cv::Size(target_width, target_height), 0, 0, cv::INTER_AREA);
  } else {
    cv::resize(decoded_img, resized_img, cv::Size(target_width, target_height), 0, 0, cv::INTER_LANCZOS4);
  }

  // Define the Region of Interest (ROI) for cropping
  cv::Rect roi(x_start, y_start, IMAGE_RESOLUTION, IMAGE_RESOLUTION);
  cv::Mat cropped_img = resized_img(roi);

  // 4. Change channel order from BGR (OpenCV default) to RGB
  cv::Mat rgb_img;
  cv::cvtColor(cropped_img, rgb_img, cv::COLOR_BGR2RGB);

  return rgb_img;
}

extern "C" {
  void mylibtorchinfer(arg_input *in, unsigned int const batch_size, arg_output *out) {slave(in,batch_size,out);}
}
