use std::{env, error::Error, fs, path::PathBuf};

use smush_ui::patch::{patch_archive, PatchArcOptions};

fn main() -> Result<(), Box<dyn Error>> {
    const BASE: &str = "src/static/layout_original.arc";
    const PATCH: &str = "src/static/layout_source";
    const OUTPUT: &str = "src/static/layout.arc";

    println!("cargo:rerun-if-changed={BASE}");
    println!("cargo:rerun-if-changed={PATCH}");
    println!("cargo:rerun-if-changed={OUTPUT}");

    let root =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").ok_or("missing CARGO_MANIFEST_DIR")?);
    let generated =
        PathBuf::from(env::var_os("OUT_DIR").ok_or("missing OUT_DIR")?).join("layout.arc");
    let output = root.join(OUTPUT);

    patch_archive(
        root.join(BASE),
        root.join(PATCH).join("patch.json"),
        &generated,
        PatchArcOptions,
    )?;

    let bytes = fs::read(&generated)?;
    if fs::read(&output).ok().as_deref() != Some(bytes.as_slice()) {
        fs::write(output, bytes)?;
    }

    Ok(())
}
