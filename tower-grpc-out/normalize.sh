# 1. cargo run --bin tower-grpc-codegen 
# 2. copy tower-grpc-out/tikvpb.rs to src/protobuf/tower_grpc_tikvpb.rs
# 3. remove all structs in src/protobuf/tower_grpc_tikvpb.rs, and add `use super::tikvpb::*`
# 4. run this script
rm tower-grpc-out/*.rs
sed -i 's/GcRequest/GCRequest/g' src/protobuf/tower_grpc_tikvpb.rs
sed -i 's/GcResponse/GCResponse/g' src/protobuf/tower_grpc_tikvpb.rs
