

fn main() -> Result<(), Box<dyn std::error::Error>>{
    tonic_build::compile_protos("protos/search.proto").unwrap();
    tonic_build::compile_protos("protos/rfcomm.proto").unwrap();
    Ok(())
}