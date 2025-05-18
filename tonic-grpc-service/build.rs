use std::env;
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    // tonic_build::configure().file_descriptor_set_path("descriptor.bin")
    //     .compile_well_known_types(true)
    //     .compile_protos("proto/grpc_pressure.proto")?;
    // tonic_build::compile_protos("proto/grpc_pressure.proto")?;
    let output_dir = env::var("OUT_DIR").unwrap_or_else(|_| ".".to_string());
    let out_dir = PathBuf::from(output_dir);
    let descriptor_path = out_dir.join("descriptor.bin");

    tonic_build::configure()
        .file_descriptor_set_path(descriptor_path)
        .compile_protos(&["proto/grpc_pressure.proto"], &["proto"])?;
    Ok(())
}
