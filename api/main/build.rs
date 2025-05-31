fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto/auth.proto");
    println!("cargo:rerun-if-changed=../proto/id.proto");
    println!("cargo:rerun-if-changed=../proto/user.proto");
    println!("cargo:rerun-if-changed=../proto/users.proto");

    tonic_build::configure()
        .build_client(false)
        .compile_protos(
            &[
                "../proto/auth.proto",
                "../proto/id.proto",
                "../proto/subscription.proto",
                "../proto/user.proto",
                "../proto/users.proto",
            ],
            &["../proto"],
        )
        .expect("failed to compile protos");

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
