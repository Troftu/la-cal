//! Generate only the project-owned ASB envelope; UCI bindings stay checked in.
fn main() {
    println!("cargo:rerun-if-changed=oms_json.proto");
    prost_build::compile_protos(&["oms_json.proto"], &["."])
        .expect("generate ASB envelope using upstream protoc on PATH");
}
