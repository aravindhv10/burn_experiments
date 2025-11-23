mod model;
mod mylib;

fn main() {
    let mut input = crate::model::arg_input {
        val: [[[0.0; model::SIZE_C as usize]; model::SIZE_Y as usize]; model::SIZE_X as usize],
    };

    let vec_input = vec![input, input, input];
    let vec_output = crate::model::run_inference(vec_input);
    println!("{:?}", vec_output);
}
