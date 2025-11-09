use tch::CModule;
use tch::Device;
use tch::Kind;
use tch::Tensor;

fn main() -> Result<(), Error> {

    let t = Tensor::from_slice(&[3, 1, 4, 1, 5]);
    let t = t * 2;
    t.print();

    let device = Device::cuda_if_available();
    let model = CModule::load_on_device("./out.pt", device)?;

    model.to_device_(device);
    println!("Model loaded successfully and moved to {:?}", device);

    println!("asd")
}
