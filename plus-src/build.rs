use cbindgen::{Builder, Language};
use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=cbindgen.toml");

    let crate_dir = env::var("CARGO_MANIFEST_DIR")?;
    let output_h = PathBuf::new()
        .join(&crate_dir)
        .join("target")
        .join("dnsmasqplus.h");

    Builder::new()
        .with_crate(crate_dir)
        .with_language(Language::C)
        .generate()?
        .write_to_file(output_h);

    Ok(())
}
