use glob::glob;
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    for entry in glob("./proto/**/*.proto")? {
        let entry = entry?;
        let entry = entry.to_str().context("entry to str failed")?;
        println!("cargo:rerun-if-changed={}", entry);
    }
    tonic_build::compile_protos("./proto/v1.proto")?;
    Ok(())
}
