fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "../proto";

    let ts_out_dir = "../front/client";
    std::fs::create_dir_all(ts_out_dir)?;

    let proto_files = std::fs::read_dir(proto_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "proto"));

    for proto_path in proto_files {
        let proto_path = proto_path.path();
        if proto_path.is_dir() {
            continue;
        }

        println!("cargo:rerun-if-changed={}", proto_path.to_str().unwrap());
        tonic_build::compile_protos(&proto_path)?;
    }

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
