extern "C" {

unsigned int constexpr IMAGE_RESOLUTION = 448;
unsigned int constexpr NUM_CHANNELS = 3;
unsigned int constexpr NUM_CLASSES = 3;

unsigned int constexpr SIZE_Y = IMAGE_RESOLUTION;
unsigned int constexpr SIZE_X = IMAGE_RESOLUTION;
unsigned int constexpr SIZE_C = NUM_CHANNELS;
unsigned int constexpr SIZE_O = NUM_CLASSES;

typedef float intype;
typedef float outtype;

struct arg_input {
  intype val[SIZE_Y][SIZE_X][SIZE_C];
};

struct arg_output {
  outtype val[SIZE_O];
};

void do_infer(arg_input *in, unsigned int const batch_size, arg_output *out);
}
