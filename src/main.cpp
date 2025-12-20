#ifndef _HEADER_GUARD_src_main_cpp
#define _HEADER_GUARD_src_main_cpp

#include "./main.hpp"

inline cv::Mat process_image_data(unsigned char *binary_data, int data_size) {
  cv::Mat decoded_img = cv::imdecode(
      cv::Mat(1, data_size, CV_8UC1, binary_data), cv::IMREAD_COLOR);

  if (decoded_img.empty()) {
    return cv::Mat::zeros(SIZE_Y, SIZE_X, CV_8UC1);
  }

  int const height = decoded_img.rows;
  int const width = decoded_img.cols;
  int target_height = IMAGE_RESOLUTION;
  int target_width = IMAGE_RESOLUTION;
  int x_start = 0;
  int y_start = 0;
  float ar = 1;

  if (height < width) {
    ar = float(width) / float(height);
    target_width = int(float(IMAGE_RESOLUTION) * ar);
    x_start = static_cast<unsigned int>(target_width - IMAGE_RESOLUTION) >> 1;
  } else {
    ar = float(height) / float(width);
    target_height = int(float(IMAGE_RESOLUTION) * ar);
    y_start = static_cast<unsigned int>(target_height - IMAGE_RESOLUTION) >> 1;
  }

  cv::Mat resized_img;
  if ((height > IMAGE_RESOLUTION) && (width > IMAGE_RESOLUTION)) {
    cv::resize(decoded_img, resized_img, cv::Size(target_width, target_height),
               0, 0, cv::INTER_AREA);

  } else {
    cv::resize(decoded_img, resized_img, cv::Size(target_width, target_height),
               0, 0, cv::INTER_LANCZOS4);

  }

  cv::Rect roi(x_start, y_start, IMAGE_RESOLUTION, IMAGE_RESOLUTION);
  cv::Mat cropped_img = resized_img(roi);

  if (false) {
    return cropped_img;
  } else {
    cv::Mat rgb_img;
    cv::cvtColor(cropped_img, rgb_img, cv::COLOR_BGR2RGB);
    return rgb_img;
  }
}

inline bool convertMatToStruct(const cv::Mat &src_mat, arg_input &dst_struct) {
  if (src_mat.rows != SIZE_Y || src_mat.cols != SIZE_X) {
    return false;
  }

  if (src_mat.type() != CV_8UC3) {
    return false;
  }

  if (src_mat.isContinuous()) {
    constexpr size_t EXPECTED_SIZE_BYTES =
        SIZE_Y * SIZE_X * SIZE_C * sizeof(intype);

    const uint8_t *mat_data_ptr = src_mat.data;

    uint8_t *struct_data_ptr = reinterpret_cast<uint8_t *>(dst_struct.val);

    std::memcpy(struct_data_ptr, mat_data_ptr, EXPECTED_SIZE_BYTES);

  } else {

    constexpr size_t ROW_SIZE_BYTES = SIZE_X * SIZE_C * sizeof(intype);

    if (false) {
      for (int y = 0; y < SIZE_Y; ++y) {
        const uint8_t *src_row = src_mat.ptr<uint8_t>(y);
        for (int x = 0; x < SIZE_X; ++x) {
          for (int c = 0; c < SIZE_C; ++c) {
            dst_struct.val[y][x][c] = src_row[(x * SIZE_C) + (SIZE_C - 1 - c)];
          }
        }
      }
    } else {
      for (int y = 0; y < SIZE_Y; ++y) {
        const uint8_t *src_row = src_mat.ptr<uint8_t>(y);
        uint8_t *dst_row = reinterpret_cast<uint8_t *>(dst_struct.val[y]);
        std::memcpy(dst_row, src_row, ROW_SIZE_BYTES);
      }
    }
  }

  return true;
}

using float32_t = float;
using float64_t = double;

template <typename T> inline auto get_tensor_dtype() {
  return torch::kBFloat16;
}

template <> inline auto get_tensor_dtype<uint8_t>() {
  return torch::kUInt8;
}

template <> inline auto get_tensor_dtype<uint16_t>() {
  return torch::kUInt16;
}

template <> inline auto get_tensor_dtype<uint32_t>() {
  return torch::kUInt32;
}

template <> inline auto get_tensor_dtype<uint64_t>() {
  return torch::kInt64;
}

template <> inline auto get_tensor_dtype<int8_t>() {
  return torch::kInt8;
}

template <> inline auto get_tensor_dtype<int16_t>() {
  return torch::kInt16;
}

template <> inline auto get_tensor_dtype<int32_t>() {
  return torch::kInt32;
}

template <> inline auto get_tensor_dtype<int64_t>() {
  return torch::kInt64;
}

template <> inline auto get_tensor_dtype<float32_t>() {
  return torch::kFloat32;
}

template <> inline auto get_tensor_dtype<float64_t>() {
  return torch::kFloat64;
}

inline std::string get_model_path() {
  printf("called get_model_path()\n");
  return std::string("/model.pt2");
}

inline torch::TensorOptions get_good_device_and_dtype() {
  printf("Called get_good_device_and_dtype()\n");
  if (torch::cuda::is_available()) {
    printf("Returning cuda");
    return torch::TensorOptions().dtype(torch::kBFloat16).device(torch::kCUDA);
  } else {
    printf("Returning cpu");
    return torch::TensorOptions().dtype(torch::kBFloat16).device(torch::kCPU);
  }
}

inline torch::TensorOptions get_host_input_device_and_dtype() {
  printf("Called get_host_input_device_and_dtype()\n");
  return torch::TensorOptions()
      .dtype(get_tensor_dtype<intype>())
      .device(torch::kCPU);
}

inline torch::TensorOptions get_host_output_device_and_dtype() {
  printf("get_host_output_device_and_dtype started\n");
  return torch::TensorOptions()
      .dtype(get_tensor_dtype<outtype>())
      .device(torch::kCPU);
}

class infer_slave {
private:
  c10::InferenceMode mode;
  torch::inductor::AOTIModelPackageLoader loader;
  torch::TensorOptions options_compute;
  torch::TensorOptions options_host_input;
  torch::TensorOptions options_host_output;
  torch::Tensor input_tensor;
  std::vector<torch::Tensor> inputs;
  std::vector<torch::Tensor> outputs;
  torch::Tensor out_tensor;
  std::size_t bytes_to_copy;

public:
  inline void operator()(arg_input *in, unsigned int const batch_size,
                         arg_output *out) {
    printf("Inside the inference function\n");
    torch::Tensor cpu_tensor = torch::from_blob(
        static_cast<void *>(in), {batch_size, SIZE_Y, SIZE_X, SIZE_C},
        options_host_input);
    printf("Step-1\n");
    inputs[0] = cpu_tensor.to(options_compute);
    printf("Step-2\n");
    outputs = loader.run(inputs);
    printf("Step-3\n");
    out_tensor = outputs[0].contiguous().cpu().to(options_host_output);
    printf("Step-4\n");
    bytes_to_copy = batch_size * SIZE_O * sizeof(outtype);
    printf("Step-5\n");
    std::memcpy(out, out_tensor.data_ptr<outtype>(), bytes_to_copy);
    printf("Step-6\n");
  }

  infer_slave()
      : loader(get_model_path()), options_compute(get_good_device_and_dtype()),
        options_host_input(get_host_input_device_and_dtype()),
        options_host_output(get_host_output_device_and_dtype()) {
    printf("Started actual constructor\n");
    inputs.resize(1);
    printf("Done constructing...\n");
  }

  ~infer_slave() {}
};

infer_slave slave;

extern "C" {

void mylibtorchinfer(arg_input *in, unsigned int const batch_size,
                     arg_output *out) {

  slave(in, batch_size, out);
}

bool decode_image_data(unsigned char *binary_data, int data_size,
                       arg_input *dst_struct) {

  /*inline*/ cv::Mat ret =
      process_image_data(/*unsigned char *binary_data =*/binary_data,
                         /*int data_size =*/data_size);

  /*inline*/ bool res = convertMatToStruct(
      /*const cv::Mat& src_mat =*/ret, /*arg_input& dst_struct =*/*dst_struct);

  return res;
}
}

#endif
