#!/usr/bin/env python3
from main import compile_model_to_aot_inductor
import sys

if __name__ == "__main__":
    compile_model_to_aot_inductor(sys.argv[1], sys.argv[2])
