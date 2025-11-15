unsafe extern "C" {
fn myfun() ;
}

fn main() {
    println!("asd") ;
    unsafe {
    myfun();
    }
}
