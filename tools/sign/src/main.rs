use clap::Parser;
use taple_core::signature::Signed;
use taple_core::{signature::Signature, EventRequest};

mod model;
use model::*;
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
    let request = {
        if args.request.starts_with('\'') || args.request.ends_with('\'') {
            remove_first_and_last_characters(&args.request)
        } else {
            args.request
        }
    };
    let request_body: EventRequestTypeBody = serde_json::from_str(&request)?;
    let request: EventRequest = request_body.clone().into();

    let signature: Signature = Signature::new_from_pk_ed25519(&request, args.private_key)?;

    let signed_event_request = Signed::<EventRequest>::new(request, signature);

    let result: String = serde_json::to_string_pretty(&signed_event_request)?;
    println!("{}", result);
    Ok(())
}

fn remove_first_and_last_characters(s: &str) -> String {
    s.chars().skip(1).take(s.len() - 2).collect()
}
