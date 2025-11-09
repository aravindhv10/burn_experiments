use tch::CModule;
use tch::Device;
use tch::Kind;
use tch::Tensor;

fn main() {

    let t = Tensor::from_slice(&[3, 1, 4, 1, 5]);
    let t = t * 2;
    t.print();

    let model = CModule::load("./out.pt")
        .context("Failed to load TorchScript model. Check the file path and LibTorch setup.")?;

    let device = Device::cuda_if_available();
    model.to_device(device);
    println!("Model loaded successfully and moved to {:?}", device);

    println!("asd")
}
