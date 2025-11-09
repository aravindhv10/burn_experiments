use std::path::Path;
use tch::CModule;
use tch::Tensor;

fn main() {

    let t = Tensor::from_slice(&[3, 1, 4, 1, 5]);
    let t = t * 2;
    t.print();

    match CModule::load(Path::new("./out.pt")) {
        Ok(model) => {
            println!("Successfully loaded the model");
        }
        Err(e) => {
            println!("Failed to load the model {}", e);
        }
    }
    println!("asd")
}
