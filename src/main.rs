pub const INPUT_SIZE: ::std::os::raw::c_ulong = 100;
pub const OUTPUT_SIZE: ::std::os::raw::c_ulong = 4;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct arg_input {
    pub val: [f32; 100usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of arg_input"][::std::mem::size_of::<arg_input>() - 400usize];
    ["Alignment of arg_input"][::std::mem::align_of::<arg_input>() - 4usize];
    ["Offset of field: arg_input::val"][::std::mem::offset_of!(arg_input, val) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct arg_output {
    pub val: [f32; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of arg_output"][::std::mem::size_of::<arg_output>() - 16usize];
    ["Alignment of arg_output"][::std::mem::align_of::<arg_output>() - 4usize];
    ["Offset of field: arg_output::val"][::std::mem::offset_of!(arg_output, val) - 0usize];
};
unsafe extern "C" {
    pub fn do_infer(
        in_: *const arg_input,
        batch_size: ::std::os::raw::c_uint,
        out: *mut arg_output,
    );
}

fn run_inference(input: Vec<arg_input>) -> Vec<arg_output> {
    let tmp = arg_output { val: [0.0; 4usize] };

    let mut output = Vec::<arg_output>::with_capacity(input.len());

    for _ in 0..input.len() {
        output.push(tmp);
    }

    unsafe {
        do_infer(input.as_ptr(), input.len() as u32, output.as_mut_ptr());
    }

    output
}

fn main() {
    let mut input = arg_input {
        val: [0.0; 100usize],
    };

    for i in 0..100 {
        input.val[i] = (i as f32) / 100.0;
    }

    let vec_input = vec![input, input, input];
    let vec_output = run_inference(vec_input);
    println!("{:?}", vec_output);
}
