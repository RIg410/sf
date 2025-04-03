fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto/auth.proto");

    tonic_build::configure()
        .build_client(false)
        .compile_protos(&["../proto/auth.proto"], &["../proto"])
        .expect("failed to compile protos");

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
