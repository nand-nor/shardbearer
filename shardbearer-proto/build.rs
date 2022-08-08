extern crate protoc_grpcio;
use std::env;

fn main() {

   // let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();

    let (herald_output, radiant_output, ctrl_output, common_output, proto_root) = {
        if target.contains("pc-windows-gnu")  {
            ("proto\\herald\\","proto\\radiant\\","proto\\bondsmith\\","proto\\","proto\\")
        } else {
            ("proto/herald/","proto/radiant/","proto/bondsmith/","proto/","proto/")
        }
    };


    println!("cargo:rerun-if-changed={}", proto_root);
    println!("cargo:rerun-if-changed={}", herald_output);
    println!("cargo:rerun-if-changed={}", radiant_output);
    println!("cargo:rerun-if-changed={}", ctrl_output);

    protoc_grpcio::compile_grpc_protos(&["common.proto"], &[proto_root], &common_output, None)
        .expect("Failure to compile common proto");

    protoc_grpcio::compile_grpc_protos(
        &["herald/herald.proto"],
        &[proto_root],
        &herald_output,
        None,
    )
    .expect("Failure to compile herald proto");
    protoc_grpcio::compile_grpc_protos(
        &["radiant/radiant.proto"],
        &[proto_root],
        &radiant_output,
        None,
    )
    .expect("Failure to compile radiant proto");

    protoc_grpcio::compile_grpc_protos(
        &["bondsmith/bondsmith.proto"],
        &[proto_root],
        &ctrl_output,
        None,
    )
    .expect("Failure to compile shard proto");
}
