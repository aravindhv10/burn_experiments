mod model;
mod mylib;
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
