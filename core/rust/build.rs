fn main() {
    println!("cargo:rerun-if-changed=../../specs/graph/graph.proto");

    prost_build::Config::new()
        .compile_protos(
            &["../../specs/graph/graph.proto"],
            &["../../specs"],
        )
        .expect("failed to compile ChangeGraph protobuf definitions");
}
