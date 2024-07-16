fn main() {
    tonic_build::compile_protos("proto/historical_data.proto").unwrap();
}
