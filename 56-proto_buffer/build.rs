fn main(){
    prost_build::Config::new()
        .out_dir("src/pb")                    // generate into src/pb
        .compile_protos(
            &["proto/employee.proto"],        // .proto files
            &["proto"],                       // include path
        )
        .unwrap();
}