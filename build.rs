use std::{fs, io};

fn main() -> io::Result<()> {
    let proto_dir = "protobufs";

    // Automatically find all .proto files in the directory
    let proto_files: Vec<String> = fs::read_dir(proto_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "proto"))
        .map(|entry| entry.path().to_string_lossy().to_string())
        .collect();

    if proto_files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No .proto files found in protobufs/",
        ));
    }

    println!("Found proto files: {proto_files:?}");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&proto_files.iter().map(|s| s.as_str()).collect::<Vec<&str>>(), &[
            proto_dir,
        ])?;

    // prevent needing to rebuild if files (or deps) haven't changed
    println!("cargo:rerun-if-changed={proto_dir}");
    Ok(())
}
