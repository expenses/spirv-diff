use spirv_tools::assembler::{compiled::CompiledAssembler, Assembler, DisassembleOptions};
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "A tool for diffing SPIR-V files")]
struct Opt {
    /// The path of the first SPIR-V file to diff
    file_a: PathBuf,
    /// The path of the second SPIR-V file to diff
    file_b: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let spirv_a = disassemble(&opt.file_a)?;
    let spirv_b = disassemble(&opt.file_b)?;

    let dir = tempfile::TempDir::new()?;

    let path_a = dir.path().join("a.dis");
    let path_b = dir.path().join("b.dis");

    std::fs::write(&path_a, &spirv_a)?;
    std::fs::write(&path_b, &spirv_b)?;

    Command::new("git")
        .arg("diff")
        .arg("--no-index")
        .arg(&path_a)
        .arg(&path_b)
        .spawn()?
        .wait()?;

    Ok(())
}

fn disassemble(path: &PathBuf) -> anyhow::Result<String> {
    let bytes = std::fs::read(path)?;
    let words = spirv_tools::binary::to_binary(&bytes)?;

    let string = CompiledAssembler::default()
        .disassemble(&words, DisassembleOptions::default())?
        .ok_or_else(|| anyhow::anyhow!("Failed to get disassembled output"))?;

    Ok(string)
}
