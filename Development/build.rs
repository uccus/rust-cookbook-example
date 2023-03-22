fn main(){
    cc::Build::new()
        .file("src/cpp/hello.c")
        .compile("hello");

    cc::Build::new()
        .cpp(true)
        .file("src/cpp/hello2.cpp")
        .compile("hello2");

}