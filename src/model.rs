include!("export.rs");

impl arg_input {
    pub fn new() -> Self {
        arg_input {
            val: [[[0.0; SIZE_C as  usize]; SIZE_X as usize]; SIZE_X as usize],
        }
    }
}

impl Default for arg_input {
    fn default() -> Self {
        arg_input::new()
    }
}

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

pub struct InferRequest {
    img: image::RgbaImage,
    resp_tx: tokio::sync::oneshot::Sender<Result<arg_output, String>>,
}

pub struct model_server {
    rx: tokio::sync::mpsc::Receiver<InferRequest>,
}

const MAX_BATCH: usize = 16;
const BATCH_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(200);

impl model_server {
    pub async fn infer_loop(&mut self) {
        while let Some(first) = self.rx.recv().await {
            let mut batch = vec![first];
            let start = tokio::time::Instant::now();
            while batch.len() < MAX_BATCH && start.elapsed() < BATCH_TIMEOUT {
                match self.rx.try_recv() {
                    Ok(req) => batch.push(req),
                    Err(_) => break,
                }
            }
            let batch_size = batch.len();

            let mut input: Vec<arg_input> = (0..batch_size).map(|_|{arg_input::new()}).collect(); 
            
            for (b, req) in batch.iter().enumerate() {
                for (x, y, pixel) in req.img.enumerate_pixels() {
                    let [r, g, b, _] = pixel.0;
                    input[b as usize].val[y as usize][x as usize][0] = r as f32;
                    input[b as usize].val[y as usize][x as usize][1] = g as f32;
                    input[b as usize].val[y as usize][x as usize][2] = b as f32;
                }
            }
            let outputs = run_inference(input) ;

            for (out, req) in outputs.into_iter().zip(batch.into_iter()) {
                let _ = req.resp_tx.send(Ok(out));
            }
        }
    }
}
