#include "./main.cpp"
int main(){
  arg_input in;
  arg_output out;
  mylibtorchinfer(/*arg_input *in =*/ &in, /*unsigned int const batch_size =*/ 1, /*arg_output *out =*/ &out);
  return 0;
}
