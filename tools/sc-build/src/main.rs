use clap::Parser;
use std::{
    error::Error,
    fs::{self, create_dir},
    process::Command, io::{self, Write},
};

mod manifest;

#[derive(Parser, Default, Debug)]
#[clap(version, about = "TAPLE Smart Contracts project generator")]
struct Args {
    /// Path where the sc folder will be generated
    pub path: String,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let path = args.path;

    let manifest = manifest::get_toml();

    std::fs::create_dir_all(&path)?;

    let cargo_path = format!("{}/Cargo.toml", path);
    fs::write(cargo_path.clone(), manifest)?;

    let src_path = format!("{}/src", path);
    create_dir(&src_path)?;

    let lib_path = format!("{}/lib.rs", src_path);

    fs::write(lib_path, "")?;

    let status = Command::new("cargo")
        .arg("build")
        .arg(format!("--manifest-path={}", cargo_path))
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release")
        .output()?;

    if !status.status.success() {
        io::stderr().write_all(&status.stderr).unwrap();
    }

    Ok(())
}
