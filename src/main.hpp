extern "C" {

unsigned long constexpr INPUT_SIZE = 100;
unsigned long constexpr OUTPUT_SIZE = 4;

struct arg_input {
  float val[INPUT_SIZE];
};

struct arg_output {
  float val[OUTPUT_SIZE];
};

arg_output do_infer(arg_input const);
}
