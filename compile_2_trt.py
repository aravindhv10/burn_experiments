#!/usr/bin/env python3
try:
    import torch_tensorrt

    HAVE_TRT = True
except:
    HAVE_TRT = False
import sys
import torch


def compile_EP_to_tensorrt(path_file_input_EP_pt2, path_file_output_trt_pt2):
    print("Inside the TRT function")
    ep = torch.export.load(path_file_input_EP_pt2)
    model = ep.module()
    x = [
        torch.randn(
            list(ep.example_inputs[0][0].size()),
            dtype=torch.bfloat16,
            device="cuda",
        )
    ]
    compile_settings = {
        "inputs": x,
        "enabled_precision": {torch.bfloat16},
        "ir": "dynamo",
    }
    trt_gm = torch_tensorrt.compile(model, **compile_settings)
    torch_tensorrt.save(
        trt_gm,
        file_path=path_file_output_trt_pt2,
        output_format="aot_inductor",
        retrace=True,
        arg_inputs=x,
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
    model.eval()
    model = model.to(
        device=device,
        dtype=dtype,
    )
    x = [
        torch.randn(
            list(ep.example_inputs[0][0].size()),
            dtype=dtype,
            device=device,
        )
    ]
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


if HAVE_TRT:
    compile_EP_to_tensorrt(
        path_file_input_EP_pt2=sys.argv[1],
        path_file_output_trt_pt2=sys.argv[2],
    )
else:
    compile_EP_to_AOTI(
        path_file_input_EP_pt2=sys.argv[1],
        path_file_output_AOTI_pt2=sys.argv[2],
    )
res = torch.export.load(sys.argv[1])
model = res.module()
# 1. Define and prepare your model and example inputs
example_inputs = [
    torch.randn(
        list(res.example_inputs[0][0].size()),
        dtype=torch.bfloat16,
        device="cuda",
    )
]
# example_inputs = res._example_inputs[0][0].cuda()
compile_settings = {
    "inputs": example_inputs,
    "enabled_precision": {torch.bfloat16},  # or torch.float32
    "ir": "dynamo",  # use the dynamo IR path
    # ... other settings
}
# 2. Compile the model with Torch-TensorRT
trt_gm = torch_tensorrt.compile(model, **compile_settings)
# 3. Save the compiled model using AOTInductor format
torch_tensorrt.save(
    trt_gm,
    file_path="model_trt.pt2",
    output_format="aot_inductor",
    retrace=True,
    arg_inputs=example_inputs,
)
