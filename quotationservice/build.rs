fn main() {
    tonic_build::compile_protos("proto/quotation.proto").unwrap();
}
