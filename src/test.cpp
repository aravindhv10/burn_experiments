#include "./main.cpp"
int main(){
  arg_input in;
  arg_output out;

  for(int y = 0 ; y < IMAGE_RESOLUTION; ++y){
    for(int x=0; x<IMAGE_RESOLUTION; ++x){
      for(int c=0; c<NUM_CHANNELS; ++c){
        in.val[y][x][c] = 0;
      }
    }
  }

  mylibtorchinfer(/*arg_input *in =*/ &in, /*unsigned int const batch_size =*/ 1, /*arg_output *out =*/ &out);
  for(int i = 0; i< NUM_CLASSES; ++i){printf("%lf, ",out.val[i]);}
  printf("\n");
  
  return 0;
}
