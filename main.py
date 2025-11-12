#!/usr/bin/env python3
import os

try:
    __file__
except:
    basepath = "."
else:
    basepath = os.path.abspath(os.path.dirname(__file__) + "/")
import sys

sys.path.append(basepath)
INPUT_SIZE = 100
BATCH_SIZE = 4
import einops
import timm
import torch


def export_to_dynamo(path_file_out):
    with torch.no_grad():
        model = model_wrapper()
        model = torch.compile(
            model=model,
            fullgraph=True,
            dynamic=True,
            backend="inductor",
            mode="max-autotune",
        )
        x = torch.rand(
            (BATCH_SIZE, INPUT_SIZE),
            dtype=torch.float32,
        )
        y = model(x)
        exported_module = torch.export.export(
            mod=model,
            args=(x,),
            strict=True,
        )

        output_path = torch._inductor.aoti_compile_and_package(
            exported_module,
            # [Optional] Specify the generated shared library path. If not specified,
            # the generated artifact is stored in your system temp directory.
            package_path=path_file_out + ".pt2",
        )

        # compiled_model = torch.compile(
        #     model=exported_module.module(),
        #     fullgraph=True,
        #     dynamic=True,
        #     backend="inductor",
        #     mode="max-autotune",
        # )

        jit_module = torch.jit.trace(
            func=exported_module.module(),
            example_inputs=x,
        )
    jit_module.save(path_file_out + ".pt")


def export_to_onnx(path_file_out):
    model = model_wrapper()
    model = torch.compile(
        model=model,
        fullgraph=True,
        dynamic=True,
        backend="inductor",
        mode="max-autotune",
    )
    x = torch.rand(
        (BATCH_SIZE, INPUT_SIZE),
        dtype=torch.float32,
    )
    y = model(x)
    res = torch.onnx.export(
        slave,
        x,
        path_file_out,
        input_names="x",
        output_names="y",
        opset_version=23,
        dynamo=True,
        external_data=True,
    )


class model_wrapper(torch.nn.Module):
    def __init__(self):
        super().__init__()
        self.L1 = torch.nn.Linear(
            in_features=INPUT_SIZE,
            out_features=4,
            bias=True,
            dtype=torch.float32,
        )

    def forward(self, x):
        x = self.L1(x)
        return x


export_to_dynamo(path_file_out="out")
