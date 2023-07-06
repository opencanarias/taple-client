use clap::Parser;
use taple_core::crypto::{Ed25519KeyPair, KeyGenerator, KeyMaterial, KeyPair};
use taple_core::identifier::{Derivable, KeyIdentifier};
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
    let key_bytes = hex::decode(args.private_key)?;
    let key_pair = KeyPair::Ed25519(Ed25519KeyPair::from_secret_key(&key_bytes));
    let request = {
        if args.request.starts_with('\'') || args.request.ends_with('\'') {
            remove_first_and_last_characters(&args.request)
        } else {
            args.request
        }
    };
    let request_body: EventRequestTypeBody = serde_json::from_str(&request)?;
    let request: EventRequest = request_body.clone().into();

    let signer = generate_identifier(&key_pair);
    let signature: Signature = Signature::new(&request, signer, &key_pair)?;

    let external_request = SignedEventRequest {
        content: request_body,
        signature: SignatureBody {
            signer: signature.signer.to_str(),
            timestamp: signature.timestamp.0,
            value: signature.value.to_str(),
        },
    };
    let result: String = serde_json::to_string_pretty(&external_request)?;
    println!("{}", result);
    Ok(())
}

fn generate_identifier(keys: &KeyPair) -> KeyIdentifier {
    KeyIdentifier::new(keys.get_key_derivator(), &keys.public_key_bytes())
}

fn remove_first_and_last_characters(s: &str) -> String {
    s.chars().skip(1).take(s.len() - 2).collect()
}
