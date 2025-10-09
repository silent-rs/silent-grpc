use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .compile_protos(&["proto/helloworld.proto"], &["proto"])?;
    println!("cargo:rerun-if-changed=proto/helloworld.proto");
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
