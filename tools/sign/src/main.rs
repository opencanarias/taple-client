use clap::Parser;

use taple_core::signature::Signed;

use taple_core::{signature::Signature, EventRequest};

#[derive(Parser, Default, Debug)]
#[clap(
    version,
    about = "TAPLE requests generator utility for invokation to TAPLE nodes"
)]

struct Args {
    /// Private key to use. HEX String format
    private_key: String,

    /// JSON String of the request data to sign
    request: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let request: EventRequest = serde_json::from_str(&args.request)?;

    let signature: Signature = Signature::new_from_pk_ed25519(&request, args.private_key)?;

    let signed_request = Signed::<EventRequest>::new(request, signature);

    let result: String = serde_json::to_string_pretty(&signed_request)?;

    println!("{}", result);

    Ok(())
}
