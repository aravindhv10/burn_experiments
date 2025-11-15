unsafe extern "C" {
    pub fn myfun();
}

fn main() {
    println!("asd") ;
    unsafe {
    myfun();
    }
}
