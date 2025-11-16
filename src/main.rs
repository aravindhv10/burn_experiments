pub const INPUT_SIZE: ::std::os::raw::c_ulong = 100;
pub const OUTPUT_SIZE: ::std::os::raw::c_ulong = 4;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct arg_input {
    pub val: [f32; 100usize],
}
// #[allow(clippy::unnecessary_operation, clippy::identity_op)]
// const _: () = {
//     ["Size of arg_input"][::std::mem::size_of::<arg_input>() - 400usize];
//     ["Alignment of arg_input"][::std::mem::align_of::<arg_input>() - 4usize];
//     ["Offset of field: arg_input::val"][::std::mem::offset_of!(arg_input, val) - 0usize];
// };
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct arg_output {
    pub val: [f32; 4usize],
}
// #[allow(clippy::unnecessary_operation, clippy::identity_op)]
// const _: () = {
//     ["Size of arg_output"][::std::mem::size_of::<arg_output>() - 16usize];
//     ["Alignment of arg_output"][::std::mem::align_of::<arg_output>() - 4usize];
//     ["Offset of field: arg_output::val"][::std::mem::offset_of!(arg_output, val) - 0usize];
// };
unsafe extern "C" {
    pub fn do_infer(arg1: *const arg_input) -> arg_output;
}

fn main() {

    let input = arg_input {
        val: [0.0; f32],
    };

    unsafe {
    let res = do_infer(&input);
        println!("{:?}",res);
    }
}
