#!/usr/bin/env python3
from main import produce_model_ep
from compile_2_trt import compile_EP_2_optimized_targer
import sys

if __name__ == "__main__":
    produce_model_ep(path_file_output_model_ep=sys.argv[1])
    compile_EP_2_optimized_targer(
        path_file_input_EP_pt2=sys.argv[1],
        path_file_output_compiled_pt2=sys.argv[2],
    )
