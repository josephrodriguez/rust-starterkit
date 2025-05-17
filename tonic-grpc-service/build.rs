use std::io::Result;
fn main() -> Result<()> {
    // tonic_build::configure().file_descriptor_set_path("descriptor.bin")
    //     .compile_well_known_types(true)
    //     .compile_protos("proto/stress_simulator.proto")?;
    tonic_build::compile_protos("proto/stress_simulator.proto")?;
    Ok(())
}
