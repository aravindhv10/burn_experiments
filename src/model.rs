include!("export.rs");

impl arg_input {
    pub fn new() -> Self {
        arg_input {
            val: [[[0 as u8; SIZE_C as  usize]; SIZE_X as usize]; SIZE_X as usize],
        }
    }

    pub fn from(mut binary_image_data: Vec<u8>) -> Result<Box<Self>, Box<Self>>  {
        let mut tmp: Box<std::mem::MaybeUninit<Self>> = Box::new_uninit();
        unsafe {
            let tmp_ptr: *mut Self = tmp.as_mut_ptr();
            let success = decode_image_data(binary_image_data.as_mut_ptr(), binary_image_data.len().try_into().unwrap(), tmp_ptr);
            if success {
                Ok(tmp.assume_init())
            } else {
                println!("Decode failed, returning 0");
                let byte_ptr = tmp_ptr as *mut u8;
                let size_in_bytes = std::mem::size_of::<Self>();
                std::ptr::write_bytes(byte_ptr, 0, size_in_bytes);
                Err(tmp.assume_init())
            }
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
    // let mut output: Vec<arg_output> = (0..input.len()).map(|_|{arg_output::new()}).collect(); 
    // unsafe {
    //     mylibtorchinfer(input.as_mut_ptr(), input.len() as u32, output.as_mut_ptr());
        
    // }
    // output


    unsafe {
        let output: *mut arg_output = mylibtorchinfer_alloc(input.as_mut_ptr(), input.len() as u32);
        if output.is_null() {
            eprintln!("C++ allocation failed or returned a null pointer.");
            return Vec::new();
        } else {
            return Vec::from_raw_parts(output, input.len(), input.len());
        }
    }
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
    img: Box<arg_input>,
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
            let mut images = Vec::with_capacity(MAX_BATCH);
            let mut reply_channel = Vec::with_capacity(MAX_BATCH);

            images.push(*(first.img));
            reply_channel.push(first.resp_tx);

            let start = tokio::time::Instant::now();
            while images.len() < MAX_BATCH && start.elapsed() < BATCH_TIMEOUT {
                match self.rx.try_recv() {
                    Ok(req) => {
                        images.push(*(req.img));
                        reply_channel.push(req.resp_tx);
                    } ,
                    Err(_) => break,
                }
            }

            let outputs = run_inference(images).await ;

            for (out, req) in outputs.into_iter().zip(reply_channel.into_iter()) {
                let _ = req.send(Ok(out));
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
        mut binary_image_data: Vec<u8>
    ) -> Result<arg_output, String> {

        let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();

        let img = match arg_input::from(binary_image_data) {
            Ok(O) => {O} ,
            Err(E) => {println!("Failed to decode image, using blind 0s");E}
        } ;

        match self.tx.send(InferRequest{ img: img, resp_tx: resp_tx}).await {
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
        return self.do_infer(data).await;
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
