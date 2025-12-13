// #[link(name="mytorch")]
include!("export.rs");

impl arg_input {
    pub fn new() -> Self {
        arg_input {
            val: [[[0.0; SIZE_C as  usize]; SIZE_X as usize]; SIZE_X as usize],
        }
    }

    pub fn from_binary_image_data(mut self: Self, mut data: Vec<u8>) -> Result<Self, Self> {
        let mut success = false;

        unsafe {
            success = decode_image_data(
                /*binary_data: *mut ::std::os::raw::c_uchar =*/ data.as_mut_ptr(),
                /*data_size: ::std::os::raw::c_int =*/ data.len().try_into().unwrap(),
                /*dst_struct: *mut arg_input =*/ &mut self
            )
        }

        if success {
            return Ok(self);
        } else {
            return Err(self);
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

async fn run_inference(mut input: Vec<arg_input>) -> Vec<arg_output> {
    let mut output: Vec<arg_output> = (0..input.len()).map(|_|{arg_output::new()}).collect(); 
    unsafe {
        mylibtorchinfer(input.as_mut_ptr(), input.len() as u32, output.as_mut_ptr());
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
            
            for (B, req) in batch.iter().enumerate() {
                for (X, Y, pixel) in req.img.enumerate_pixels() {
                    let [r, g, b, _] = pixel.0;
                    input[B as usize].val[Y as usize][X as usize][0] = r as f32;
                    input[B as usize].val[Y as usize][X as usize][1] = g as f32;
                    input[B as usize].val[Y as usize][X as usize][2] = b as f32;
                }
            }
            let outputs = run_inference(input).await ;

            for (out, req) in outputs.into_iter().zip(batch.into_iter()) {
                let _ = req.resp_tx.send(Ok(out));
            }
        }
    }
}

pub struct model_client {
    tx: tokio::sync::mpsc::Sender<InferRequest>,
    preprocess: crate::mylib::image_processor,
}

impl model_client {
    pub async fn do_infer(
        &self,
        img: image::RgbaImage,
    ) -> Result<arg_output, String> {

        let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();

        match self.tx.send(InferRequest { img, resp_tx}).await {
            Ok(_) => match resp_rx.await {
                Ok(Ok(pred)) => {
                    return Ok(pred);
                }
                Ok(Err(e)) => {
                    return Err(e);
                }
                Err(e) => {
                    return Err("Recv Error".to_string());
                }
            },
            Err(e) => {
                return Err("Send error".to_string());
            }
        }
    }

    pub async fn do_infer_data(&self, data: Vec<u8>) -> Result<arg_output, String> {

        match self.preprocess.decode_and_preprocess(data) {
            Ok(img) => {
                return self.do_infer(img).await;
            }
            Err(e) => {
                return Err("Failed to decode and pre-process the image".to_string());
            }
        }
    }
}

pub fn get_inference_tuple() -> (model_server, model_client) {
    let (tx, rx) = tokio::sync::mpsc::channel::<InferRequest>(512);
    let ret_server = model_server {
        rx: rx,
    };
    let ret_client = model_client {
        tx: tx,
        preprocess: crate::mylib::image_processor::new(IMAGE_RESOLUTION),
    };
    return (ret_server, ret_client);
}
