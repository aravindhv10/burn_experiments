unsafe extern "C" {
    pub fn torchmain();
}

fn main() {
    println!("asd") ;
    unsafe {
    torchmain();
    }
}
