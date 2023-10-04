use clap::{Parser, ValueEnum};

use serde::{Deserialize, Serialize};

use taple_core::{signature::Signature, EventRequest};

#[derive(Parser, Default, Debug)]
#[clap(
    version,
    about = "TAPLE requests generator utility for invokation to TAPLE nodes"
)]
struct Args {
    /// Private key to use. HEX String format
    #[arg(short = 'k', long = "id-private-key", required = true)]
    private_key: String,

    /// JSON String of the request data to sign
    request: String,

    /// Key derivator to use.
    #[arg(value_enum, long, default_value_t = KeyDerivator::Ed25519)]
    derivator: KeyDerivator,

    /// Digest derivator to use
    #[arg(value_enum, long, default_value_t = DigestDerivator::Blake3_256)]
    digest_derivator: DigestDerivator,
}

/// Key derivators availables
#[derive(ValueEnum, Clone, Debug)]
enum KeyDerivator {
    /// The Ed25519 key derivator.
    Ed25519,
    /// The Secp256k1 key derivator.
    Secp256k1,
}

/// Key derivators availables
#[derive(ValueEnum, Clone, Debug)]
pub enum DigestDerivator {
    Blake3_256,
    Blake3_512,
    SHA2_256,
    SHA2_512,
    SHA3_256,
    SHA3_512,
}

impl Into<taple_core::DigestDerivator> for DigestDerivator {
    fn into(self) -> taple_core::DigestDerivator {
        match self {
            DigestDerivator::Blake3_256 => taple_core::DigestDerivator::Blake3_256,
            DigestDerivator::Blake3_512 => taple_core::DigestDerivator::Blake3_512,
            DigestDerivator::SHA2_256 => taple_core::DigestDerivator::SHA2_256,
            DigestDerivator::SHA2_512 => taple_core::DigestDerivator::SHA2_512,
            DigestDerivator::SHA3_256 => taple_core::DigestDerivator::SHA3_256,
            DigestDerivator::SHA3_512 => taple_core::DigestDerivator::SHA3_512,
        }
    }
}

impl Default for KeyDerivator {
    fn default() -> Self {
        Self::Ed25519
    }
}

impl Default for DigestDerivator {
    fn default() -> Self {
        Self::Blake3_256
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedRequest {
    pub request: EventRequest,
    pub signature: Signature,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let request: EventRequest = serde_json::from_str(&args.request)?;

    let signature = match args.derivator {
        KeyDerivator::Ed25519 => Signature::new_from_pk_ed25519(
            &request,
            args.private_key,
            args.digest_derivator.into(),
        )?,
        KeyDerivator::Secp256k1 => Signature::new_from_pk_secp256k1(
            &request,
            args.private_key,
            args.digest_derivator.into(),
        )?,
    };

    let signed_request = SignedRequest { request, signature };

    let result: String = serde_json::to_string_pretty(&signed_request)?;

    println!("{}", result);

    Ok(())
}
