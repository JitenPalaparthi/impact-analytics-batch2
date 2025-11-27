// build.rs
fn main() {
    cc::Build::new()
        .file("c_src/mylib.c") // our C file
        .include("c_src")      // so it can find "mylib.h"
        .compile("mylib");     // name of the generated static library libmylib.a
}
// clang -c mylib.c -o mylib.o
// clang --> llvm ir --> as --> .o file 
// rustc --> llvm ir --> as --> .o file -> linker --> bin
// goc --> .o file --> .o file +. other files --> bin

// javac --> .class --> .jar jvm(jit)--> bin 
// scalac --> 
// closurec --> 
// kotlinc -->


// csharpc --> il code --> .net runtime (JIT)
// fsharpc 
// vb.

// ar rcs libmylib.a mylib.o   