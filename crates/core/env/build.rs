use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let start = SystemTime::now();
    let duration = start.duration_since(UNIX_EPOCH).unwrap();
    let timestamp = duration.as_secs();

    println!("cargo:rustc-env=BUILD_TIME={timestamp}");
}
