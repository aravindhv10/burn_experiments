include!("export.rs");

const CLASS_LABELS: [&str; SIZE_O as usize] = ["empty", "occupied", "other"];

impl arg_output {

    pub fn new() -> Self {
        arg_output {
            val: [0.0; SIZE_O as usize],
        }
    }

    pub fn from<T: std::ops::Index<usize, Output = outtype>>(input: T) -> Self {
        let mut ret = arg_output::new();
        for i in 0..SIZE_O {
            ret.val[i as usize] = input[i as usize];
        }
        ret
    }

}

impl Default for arg_output {
    fn default() -> Self {
        arg_output::new()
    }
}

pub fn run_inference(input: Vec<arg_input>) -> Vec<arg_output> {

    let mut output: Vec<arg_output> = (0..input.len()).map(|_|{arg_output::new()}).collect(); 

    unsafe {
        do_infer(input.as_ptr(), input.len() as u32, output.as_mut_ptr());
    }

    output
}

#[derive(serde::Serialize)]
pub struct prediction_probabilities_reply {
    val: [String; SIZE_O as usize],
    mj: String,
}

impl prediction_probabilities_reply {

    pub fn new() -> Self {
        prediction_probabilities_reply {
            val: std::array::from_fn(|_| String::new()),
            mj: String::new(),
        }
    }

    pub fn from(input: arg_output) -> prediction_probabilities_reply {
        let mut max_index: usize = 0;
        let mut ret = prediction_probabilities_reply::new();

        ret.val[0] = input.val[0].to_string();
        for i in 1..SIZE_O {
            ret.val[i as usize] = input.val[i as usize].to_string();
            if input.val[i as usize] > input.val[max_index] {
                max_index = i as usize;
            }
        }

        ret.mj = CLASS_LABELS[max_index].to_string();

        return ret;
    }
}

use tokio;

pub struct InferRequest {
    img: image::RgbaImage,
    resp_tx: tokio::sync::oneshot::Sender<Result<arg_output, String>>,
}
