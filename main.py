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
from torch.export.dynamic_shapes import Dim


def produce_model(path_file_out):
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

        dynamic_shapes = {
            "x": (Dim.DYNAMIC, Dim.STATIC),
        }
        exported_module = torch.export.export(
            model._orig_mod,
            (x,),
            dynamic_shapes=dynamic_shapes,
            # strict=True,
            # dynamic_shapes=dynamic_shapes,
        )

        torch.export.save(
            ep=exported_module,
            f=path_file_out + ".pt2",
        )


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

    def forward(
        self,
        x: torch.Tensor,
    ):
        x = self.L1(x)
        return x


produce_model(path_file_out="model_input")
