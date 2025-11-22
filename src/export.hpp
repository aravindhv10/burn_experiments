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
  float val[SIZE_Y][SIZE_X][SIZE_C];
};

struct arg_output {
  float val[SIZE_O];
};

void do_infer(arg_input const *in, unsigned int const batch_size, arg_output *out);
}
