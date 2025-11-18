fn main() {
    let library_path = std::path::Path::new("/usr/include/torch/csrc/api/include/");

    cc::Build::new()
        .cpp(true)
        .file("src/main.cpp")
        .include(library_path)
        .flag("-Ofast")
        .flag("-mtune=native")
        .flag("-march=native")
        .compile("main");

    println!("cargo:rustc-link-arg=-ltorch");
    println!("cargo:rustc-link-arg=-lc10");
    println!("cargo:rustc-link-arg=-ltorch_cpu");
    // println!("cargo:rustc-link-arg=-Wl,--no-as-needed");
    // println!("cargo:rustc-link-arg=-laoti_custom_ops");
    // println!("cargo:rustc-link-arg=-lgomp-98b21ff3");
    // println!("cargo:rustc-link-arg=-lbackend_with_compiler");
    // println!("cargo:rustc-link-arg=-ljitbackend_test");
    // println!("cargo:rustc-link-arg=-lnnapi_backend");
    // println!("cargo:rustc-link-arg=-lshm");
    // println!("cargo:rustc-link-arg=-ltorch");
    // println!("cargo:rustc-link-arg=-ltorch_global_deps");
    // println!("cargo:rustc-link-arg=-ltorch_python");
    // println!("cargo:rustc-link-arg=-ltorchbind_test");
}
