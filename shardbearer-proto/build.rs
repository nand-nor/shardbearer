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
                "proto\\herald\\herald.proto",
                "proto\\herald\\",
                "proto\\radiant\\radiant.proto",
                "proto\\radiant\\",
                "proto\\bondsmith\\bondsmith.proto",
                "proto\\bondsmith\\",
                "proto\\common.proto",
                "proto\\",
                "proto\\"
            )
        } else {
            (
                "proto/herald/herald.proto",
                "proto/herald/",
                "proto/radiant/radiant.proto",
                "proto/radiant/",
                "proto/bondsmith/bondsmith.proto",
                "proto/bondsmith/",
                "proto/common.proto",
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
/*
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: herald_output,
        includes: &[proto_root],
        input: &[herald_input],
        rust_protobuf: true, // also generate protobuf messages, not just services
        ..Default::default()
    }).expect("protoc-rust-grpc");

    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: radiant_output,
        includes: &[proto_root],
        input: &[radiant_input],
        rust_protobuf: true, // also generate protobuf messages, not just services
        ..Default::default()
    }).expect("protoc-rust-grpc");

    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: bondsmith_output,
        includes: &[proto_root],
        input: &[bondsmith_input],
        rust_protobuf: true, // also generate protobuf messages, not just services
        ..Default::default()
    }).expect("protoc-rust-grpc");

    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: bondsmith_output,
        includes: &[proto_root],
        input: &[bondsmith_input],
        rust_protobuf: true, // also generate protobuf messages, not just services
        ..Default::default()
    }).expect("protoc-rust-grpc");

    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: common_output,
        includes: &[proto_root],
        input: &[common_input],
        rust_protobuf: true, // also generate protobuf messages, not just services
        ..Default::default()
    }).expect("protoc-rust-grpc");
*/

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
            &bondsmith_output,
            None,
        )
        .expect("Failure to compile shard proto");


}
