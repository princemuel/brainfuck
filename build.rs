fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto = "protobufs/authentication.proto";

    tonic_build::compile_protos(proto)?;

    // prevent needing to rebuild if files (or deps) haven't changed
    println!("cargo:rerun-if-changed={proto}");
    Ok(())
}
