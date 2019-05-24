extern crate tower_grpc_build;
use std::env;

fn main() {
    env::set_var("OUT_DIR", "./tower-grpc-out");
    // Build helloworld
    tower_grpc_build::Config::new()
        .enable_server(true)
        .enable_client(true)
        .build(&["proto/tower-grpc/tikvpb.proto", 
        ], &["proto/tower-grpc"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
