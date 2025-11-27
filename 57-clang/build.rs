fn main() {
    cc::Build::new()
        .file("c_code/mylib.c") // our C file
        .include("c_code")      // so it can find "mylib.h"
        .compile("mylib");     // name of the generated static library libmylib.a
}