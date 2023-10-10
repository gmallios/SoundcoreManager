fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile the proto files in the protos directory
    // only on macOS
    if cfg!(target_os = "macos") {
        tonic_build::compile_protos("protos/search.proto").unwrap();
        tonic_build::compile_protos("protos/rfcomm.proto").unwrap();
    }
    Ok(())
}
