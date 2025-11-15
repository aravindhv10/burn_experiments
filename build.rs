fn main() {
cc::Build::new()
    .cpp(true)
    .file("main.cpp")
    .compile("main");
}
