#!/usr/bin/env python3
import os
import sys
import torch


def compile_to_dynamo(path_file_in, path_file_out):
    exported_module = torch.export.load(f=path_file_in)
    output_path = torch._inductor.aoti_compile_and_package(
        exported_module,
        package_path=path_file_out,
    )


compile_to_dynamo(
    path_file_in=sys.argv[1],
    path_file_out=sys.argv[2],
)
