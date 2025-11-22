include!("export.rs");

use std::ops::Index;
impl arg_output {
    pub fn new() -> Self {
        arg_output {
            val: [0.0; SIZE_O as usize],
        }
    }
    pub fn from<T: Index<usize, Output = outtype>>(input: T) -> Self {
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
    // my_vec will be: [0, 1, 2, 3, 4]


    // let tmp = arg_output { val: [0.0; SIZE_O as usize] };

    // let mut output = Vec::<arg_output>::with_capacity(input.len());

    // for _ in 0..input.len() {
    //     output.push(tmp);
    // }

    unsafe {
        do_infer(input.as_ptr(), input.len() as u32, output.as_mut_ptr());
    }

    output
}
