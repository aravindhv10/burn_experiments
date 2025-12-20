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
try:
    import torch_tensorrt

    HAVE_TRT = True
except:
    HAVE_TRT = False
import torch


def compile_EP_to_tensorrt(
    path_file_input_EP_pt2,
    path_file_output_trt_pt2,
):
    device = "cuda"
    dtype = torch.bfloat16
    with torch.no_grad():
        EP = torch.export.load(path_file_input_EP_pt2)
        model = EP.module().to(
            device=device,
            dtype=dtype,
        )
        main_shape = tuple(EP.example_inputs[0][0].size())
        min_shape = tuple([1] + [main_shape[i] for i in range(1, len(main_shape))])
        opt_shape = tuple([8] + [main_shape[i] for i in range(1, len(main_shape))])
        max_shape = tuple([32] + [main_shape[i] for i in range(1, len(main_shape))])
        example_inputs = (
            torch.randn(
                opt_shape,
                device=device,
                dtype=dtype,
            ),
        )
        batch_dim = torch.export.Dim(
            "batch",
            min=1,
            max=32,
        )
        # [Optional] Specify the first dimension of the input x as dynamic.
        exported = torch.export.export(
            model,
            example_inputs,
            dynamic_shapes=({0: batch_dim},),
        )
        # [Note] In this example we directly feed the exported module to aoti_compile_and_package.
        # Depending on your use case, e.g. if your training platform and inference platform
        # are different, you may choose to save the exported model using torch.export.save and
        # then load it back using torch.export.load on your inference platform to run AOT compilation.
        compile_settings = {
            "arg_inputs": [
                torch_tensorrt.Input(
                    min_shape=min_shape,
                    opt_shape=opt_shape,
                    max_shape=max_shape,
                    dtype=dtype,
                )
            ],
            "enabled_precisions": {dtype},
            "min_block_size": 1,
        }
        cg_trt_module = torch_tensorrt.dynamo.compile(exported, **compile_settings)
        torch_tensorrt.save(
            cg_trt_module,
            file_path=path_file_output_trt_pt2,
            output_format="aot_inductor",
            retrace=True,
            arg_inputs=example_inputs,
        )


def compile_EP_to_AOTI(
    path_file_input_EP_pt2,
    path_file_output_AOTI_pt2,
):
    print("Inside the AOTI function")
    device = "cpu"
    dtype = torch.bfloat16
    inductor_configs = {}
    if torch.cuda.is_available():
        device = "cuda"
        inductor_configs["max_autotune"] = True
    ep = torch.export.load(path_file_input_EP_pt2)
    model = ep.module()
    model = model.to(
        device=device,
        dtype=dtype,
    )
    x = torch.randn(
        list(ep.example_inputs[0][0].size()),
        dtype=dtype,
        device=device,
    )
    dynamic_shapes = {
        "x": (
            torch.export.dynamic_shapes.Dim.DYNAMIC,
            torch.export.dynamic_shapes.Dim.STATIC,
            torch.export.dynamic_shapes.Dim.STATIC,
            torch.export.dynamic_shapes.Dim.STATIC,
        ),
    }
    exported_program = torch.export.export(
        model,
        (x,),
        dynamic_shapes=dynamic_shapes,
        strict=True,
    )
    path = torch._inductor.aoti_compile_and_package(
        exported_program,
        package_path=path_file_output_AOTI_pt2,
        inductor_configs=inductor_configs,
    )


def compile_EP_2_optimized_targer(
    path_file_input_EP_pt2,
    path_file_output_compiled_pt2,
):
    if HAVE_TRT:
        compile_EP_to_tensorrt(
            path_file_input_EP_pt2=path_file_input_EP_pt2,
            path_file_output_trt_pt2=path_file_output_compiled_pt2,
        )
    else:
        compile_EP_to_AOTI(
            path_file_input_EP_pt2=path_file_input_EP_pt2,
            path_file_output_AOTI_pt2=path_file_output_compiled_pt2,
        )


if __name__ == "__main__":
    compile_EP_to_AOTI(sys.argv[1], sys.argv[2])
