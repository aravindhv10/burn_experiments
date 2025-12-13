#include "./main.hpp"

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

  cv::Rect roi(x_start, y_start, IMAGE_RESOLUTION, IMAGE_RESOLUTION);
  cv::Mat cropped_img = resized_img(roi);
  return cropped_img;

  // cv::Mat rgb_img;
  // // cv::cvtColor(cropped_img, rgb_img, cv::COLOR_BGR2RGB);
  // return rgb_img;
}

inline bool convertMatToStruct(const cv::Mat& src_mat, arg_input& dst_struct) {
    if (src_mat.rows != SIZE_Y || src_mat.cols != SIZE_X) {return false;}
    if (src_mat.type() != CV_8UC3) {return false;}
    for (int y = 0; y < SIZE_Y; ++y) {
        const uint8_t* src_row = src_mat.ptr<uint8_t>(y);
        for (int x = 0; x < SIZE_X; ++x) {
          for(int c=0; c<SIZE_C; ++c){
            dst_struct.val[y][x][c] = src_row[(x*SIZE_C) + (SIZE_C-1-c)];
          }
      }
    }
    return true;
}

inline torch::TensorOptions get_good_device_and_dtype(){
    if (torch::cuda::is_available()) {
        return torch::TensorOptions().dtype(torch::kBFloat16).device(torch::kCUDA);
    } else {
        return torch::TensorOptions().dtype(torch::kBFloat16).device(torch::kCPU);
    }
}

inline torch::TensorOptions get_host_device_and_dtype(){
    return torch::TensorOptions().dtype(torch::kFloat32).device(torch::kCPU);
}

class infer_slave {
  c10::InferenceMode mode;
  torch::inductor::AOTIModelPackageLoader loader;
  torch::TensorOptions options;
  torch::TensorOptions options_host;
  torch::Tensor input_tensor;
  std::vector<torch::Tensor> inputs;
  std::vector<torch::Tensor> outputs;
  torch::Tensor out_tensor;
  std::size_t bytes_to_copy;

public:
  inline void operator()(arg_input *in, unsigned int const batch_size, arg_output *out) {
    torch::Tensor cpu_tensor = torch::from_blob(static_cast<void *>(in), {batch_size, SIZE_Y, SIZE_X, SIZE_C}, torch::kCPU);
    inputs[0] = cpu_tensor.to(options);
    outputs = loader.run(inputs);
    out_tensor = outputs[0].contiguous().cpu().to(options_host);
    bytes_to_copy = batch_size * SIZE_O * sizeof(outtype);
    std::memcpy(out, out_tensor.data_ptr<outtype>(), bytes_to_copy);
  }

  infer_slave()
      : loader("/model.pt2"),
        options(get_good_device_and_dtype()),
        options_host(get_host_device_and_dtype()) {
    inputs.resize(1);
  }


  ~infer_slave() {}
};

infer_slave slave;


extern "C" {
  void mylibtorchinfer(arg_input *in, unsigned int const batch_size, arg_output *out) {slave(in,batch_size,out);}
  bool decode_image_data(unsigned char *binary_data, int data_size, arg_input * dst_struct){
    /*inline*/ cv::Mat ret = process_image_data(/*unsigned char *binary_data =*/ binary_data, /*int data_size =*/ data_size) ;
    /*inline*/ bool res = convertMatToStruct(/*const cv::Mat& src_mat =*/ ret, /*arg_input& dst_struct =*/ *dst_struct) ;
    return res;
  }
}
