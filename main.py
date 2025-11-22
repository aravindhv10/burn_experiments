import os

try:
    __file__
except:
    basepath = "."
else:
    basepath = os.path.abspath(os.path.dirname(__file__) + "/")
import sys

sys.path.append(basepath)
IMAGE_RESOLUTION = 448
SIZE_X = IMAGE_RESOLUTION
SIZE_Y = IMAGE_RESOLUTION
SIZE_C = 3
BATCH_SIZE = 4
INPUT_SHAPE = (BATCH_SIZE, SIZE_Y, SIZE_X, SIZE_C)
from torch.export.dynamic_shapes import Dim
import einops
import timm
import torch


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


class model_wrapper(torch.nn.Module):
    def setup_stat_parameters_as_float(self):
        self.mean = (
            0.48145466,
            0.4578275,
            0.40821073,
        )
        self.std = (
            0.26862954,
            0.26130258,
            0.27577711,
        )

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
