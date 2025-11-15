extern "C" {

struct arg_input {
  float val[100];
};

struct arg_output {
  float val[4];
};

arg_output torchmain(arg_input);
}
