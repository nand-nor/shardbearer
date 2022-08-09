extern crate protoc_grpcio;
use std::env;
//extern crate protoc_rust_grpc;

fn main() {

    // let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();

    let (
        herald_input,
        herald_output,
        radiant_input,
        radiant_output,
        bondsmith_input,
        bondsmith_output,
        common_input,
        common_output,
        proto_root
    ) = {
        if target.contains("pc-windows-gnu") {
            (
                "herald\\herald.proto",
                "proto\\herald\\",
                "radiant\\radiant.proto",
                "proto\\radiant\\",
                "bondsmith\\bondsmith.proto",
                "proto\\bondsmith\\",
                "common.proto",
                "proto\\",
                "proto\\"
            )
        } else {
            (
                "herald/herald.proto",
                "proto/herald/",
                "radiant/radiant.proto",
                "proto/radiant/",
                "bondsmith/bondsmith.proto",
                "proto/bondsmith/",
                "common.proto",
                "proto/",
                "proto/"
            )
        }
    };


    println!("cargo:rerun-if-changed={}", proto_root);
    println!("cargo:rerun-if-changed={}", herald_output);
    println!("cargo:rerun-if-changed={}", radiant_output);
    println!("cargo:rerun-if-changed={}", bondsmith_output);
    println!("cargo:rerun-if-changed={}", common_output);


        protoc_grpcio::compile_grpc_protos(&[common_input], &[proto_root], &common_output, None)
            .expect("Failure to compile common proto");

        protoc_grpcio::compile_grpc_protos(
            &[herald_input],
            &[proto_root],
            &herald_output,
            None,
        )
        .expect("Failure to compile herald proto");
        protoc_grpcio::compile_grpc_protos(
            &[radiant_input],
            &[proto_root],
            &radiant_output,
            None,
        )
        .expect("Failure to compile radiant proto");

        protoc_grpcio::compile_grpc_protos(
            &[bondsmith_input],
            &[proto_root],
            &bondsmith_output,
            None,
        )
        .expect("Failure to compile shard proto");


}
