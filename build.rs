extern crate protobuf_codegen;
extern crate protoc_grpcio;
use protobuf_codegen::Customize;

fn main() {
    // let proto_root = "src/protos";
    // println!("cargo:rerun-if-changed={}", proto_root);
    // protoc_grpcio::compile_grpc_protos(
    //     &["invoice.proto"],
    //     &[proto_root],
    //     &proto_root,
    //     Some(Customize {
    //         serde_derive: Some(true),
    //         serde_derive_cfg: None,
    //         ..Default::default()
    //     }),
    // )
    // .expect("Failed to compile gRPC definitions!");
}
