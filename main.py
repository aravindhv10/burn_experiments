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
IMAGE_RESOLUTION = 448
BATCH_SIZE = 4
NUM_CHANNELS = 3
NUM_CLASSES = 3
(
    SIZE_B,
    SIZE_Y,
    SIZE_X,
    SIZE_C,
) = (
    BATCH_SIZE,
    IMAGE_RESOLUTION,
    IMAGE_RESOLUTION,
    NUM_CHANNELS,
)
INPUT_SHAPE = (
    SIZE_B,
    SIZE_Y,
    SIZE_X,
    SIZE_C,
)
from torch.export.dynamic_shapes import Dim
import einops
import os
import sys
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
            INPUT_SHAPE,
            dtype=torch.float32,
        )
        model.to("cuda")
        x = x.to("cuda")
        dynamic_shapes = {
            "x": (
                Dim.DYNAMIC,
                Dim.STATIC,
                Dim.STATIC,
                Dim.STATIC,
            ),
        }
        exported_module = torch.export.export(
            model._orig_mod,
            (x,),
            dynamic_shapes=dynamic_shapes,
            strict=True,
        )
        exported_module = torch.export.load(f=path_file_in)
        output_path = torch._inductor.aoti_compile_and_package(
            exported_module,
            package_path=path_file_out,
        )
        # torch.export.save(
        #     ep=exported_module,
        #     f=path_file_out,
        # )


class model_wrapper(torch.nn.Module):
    ################################################################
    ## Forward related functions BEGIN #############################
    ################################################################
    def forward_1_rearrange(
        self,
        x: torch.Tensor,
    ):
        x = einops.rearrange(
            x,
            "B Y X C -> B C Y X",
        )
        return x

    def forward_2_normalize(
        self,
        x: torch.Tensor,
    ):
        for i in range(SIZE_C):
            x[:, i, :, :] = ((x[:, i, :, :] / 255.0) - self.mean[i]) / self.std[i]
        return x

    def forward_3_backbone(
        self,
        x: torch.Tensor,
    ):
        x = self.timm_model(x)
        return x

    def forward_4_postprocess(
        self,
        x: torch.Tensor,
    ):
        x = torch.nn.functional.softmax(
            x,
            dim=1,
        )
        return x

    def forward(
        self,
        x: torch.Tensor,
    ):
        x = self.forward_1_rearrange(x)
        x = self.forward_2_normalize(x)
        x = self.forward_3_backbone(x)
        x = self.forward_4_postprocess(x)
        return x

    ################################################################
    ## Forward related functions END ###############################
    ################################################################
    ################################################################
    ## Init related functions BEGIN ################################
    ################################################################
    def init_setup_stat_parameters_as_float(self):
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

    def init_timm_model(self):
        self.timm_model = timm.create_model(
            "timm/eva02_base_patch14_448.mim_in22k_ft_in1k",
            num_classes=NUM_CLASSES,
            pretrained=True,
        )

    def __init__(self):
        super().__init__()
        self.init_setup_stat_parameters_as_float()
        self.init_timm_model()

    ################################################################
    ## Init related functions END ##################################
    ################################################################


produce_model(path_file_out=sys.argv[1])
