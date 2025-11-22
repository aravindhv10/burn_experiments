extern "C" {

unsigned long constexpr IMAGE_RESILUTION = 448;
unsigned long constexpr NUM_CHANNELS = 3;
unsigned long constexpr NUM_CLASSES = 3;

unsigned long constexpr SIZE_Y = IMAGE_RESILUTION;
unsigned long constexpr SIZE_X = IMAGE_RESILUTION;
unsigned long constexpr SIZE_C = NUM_CHANNELS;

unsigned long constexpr SIZE_O = NUM_CLASSES;

struct arg_input {
  float val[SIZE_Y][SIZE_X][SIZE_C];
};

struct arg_output {
  float val[SIZE_O];
};

void do_infer(arg_input const *in, unsigned int batch_size, arg_output *out);
}
