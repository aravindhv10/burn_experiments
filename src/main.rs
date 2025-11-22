mod model;
mod mylib;
fn main() {
    let mut input = crate::model::arg_input {
        val: [[[0.0; 3usize]; 448usize]; 448usize],
    };

    let vec_input = vec![input, input, input];
    let vec_output = crate::model::run_inference(vec_input);
    println!("{:?}", vec_output);
}
