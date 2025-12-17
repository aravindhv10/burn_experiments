#ifndef _HEADER_GUARD_src_export_hpp_
#define _HEADER_GUARD_src_export_hpp_

extern "C" {

unsigned int constexpr IMAGE_RESOLUTION = 448;
unsigned int constexpr NUM_CHANNELS = 3;
unsigned int constexpr NUM_CLASSES = 3;
unsigned int constexpr SIZE_B = 16;

unsigned int constexpr SIZE_Y = IMAGE_RESOLUTION;
unsigned int constexpr SIZE_X = IMAGE_RESOLUTION;
unsigned int constexpr SIZE_C = NUM_CHANNELS;
unsigned int constexpr SIZE_O = NUM_CLASSES;

typedef unsigned char intype;
typedef float outtype;

struct arg_input {
  intype val[SIZE_Y][SIZE_X][SIZE_C];
};

struct arg_output {
  outtype val[SIZE_O];
};

void mylibtorchinfer(arg_input *in, unsigned int const batch_size,
                     arg_output *out);

bool decode_image_data(unsigned char *binary_data, int data_size,
                       arg_input *dst_struct);

arg_output const *mylibtorchinfer_alloc(arg_input *in, unsigned int const batch_size);
}

#endif
