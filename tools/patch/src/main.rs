use std::error::Error;
mod error;

use clap::Parser;
use json_patch::diff;
use serde_json::Value;

use crate::error::TaplePatchError;

#[derive(Parser, Default, Debug)]
#[clap(version, about = "TAPLE JSON PATCH generator")]
struct Args {
    /// JSON String of the current state
    pub state: String,
    /// JSON String of the desired new state
    pub new_state: String,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let state: Value = serde_json::from_str(&args.state)
        .map_err(|_| TaplePatchError::InvalidJSON("state".into()))?;
    let new_state: Value = serde_json::from_str(&args.new_state)
        .map_err(|_| TaplePatchError::InvalidJSON("new_state".into()))?;
    let patch = diff(&state, &new_state);
    let result: String = serde_json::to_string_pretty(&patch)?;
    println!("{}", result);
    Ok(())
}
