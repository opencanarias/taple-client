use std::str::FromStr;

use clap::{Parser, ValueEnum};
use libp2p::core::PeerId;
use libp2p::identity::{ed25519::Keypair as EdKeyPair, secp256k1::SecretKey};
use taple_core::crypto::{Ed25519KeyPair, KeyGenerator, KeyMaterial, KeyPair, Secp256k1KeyPair};
use taple_core::identifier::{Derivable, KeyIdentifier};

#[derive(Parser, Default, Debug)]
#[command(override_help = "
    MC generation utility for TAPLE nodes\n
\x1b[1m\x1b[4mUsage\x1b[0m: taple-keygen [OPTIONS] \n
\x1b[1m\x1b[4mOptions\x1b[0m:
    \x1b[1m-m, --mode\x1b[0m           Algorithm to use: ed25519 (default), secp256k1
    \x1b[1m-f, --format\x1b[0m         Output format: yaml(default), json
    \x1b[1m-h, --help\x1b[0m           Print help information
    \x1b[1m-V, --version\x1b[0m        Print version information  
    ")]
#[clap(version)]
struct Args {
    /// Algorithm to use. Default to Ed25519
    #[clap(value_enum, short = 'm', long = "mode")]
    mode: Option<Algorithm>,
    #[clap(short = 'f', long = "format")]
    format: Option<Format>,
}

#[derive(Parser, Clone, Debug, ValueEnum, Default)]
enum Algorithm {
    #[default]
    Ed25519,
    Secp256k1,
}

#[derive(Parser, Clone, Debug, ValueEnum, Default)]
enum Format {
    #[default]
    Yaml,
    Json,
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "json" => Ok(Format::Json),
            "yaml" => Ok(Format::Yaml),
            _ => Err(format!("'{}' is not a valid format", s)),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();
    let format = args.format.unwrap_or(Format::Yaml);
    let (kp, peer_id) = match args.mode.unwrap_or(Algorithm::Ed25519) {
        Algorithm::Ed25519 => {
            let keys = generate_ed25519();
            let peer_id = PeerId::from_public_key(
                &libp2p::identity::Keypair::Ed25519(
                    EdKeyPair::decode(&mut keys.to_bytes()).expect("Decode of Ed25519 possible"),
                )
                .public(),
            );
            let keys = KeyPair::Ed25519(keys);
            (keys, peer_id)
        }
        Algorithm::Secp256k1 => {
            let keys = generate_secp256k1();
            let peer_id = PeerId::from_public_key(
                &libp2p::identity::Keypair::Secp256k1(
                    SecretKey::from_bytes(&mut keys.secret_key_bytes())
                        .expect("Be a valid Secp256k1 secret key")
                        .into(),
                )
                .public(),
            );
            let keys = KeyPair::Secp256k1(keys);
            (keys, peer_id)
        }
    };

    show_data(kp, peer_id, format);
    Ok(())
}

fn show_data(kp: KeyPair, peer_id: PeerId, format: Format) {
    let private_key = kp.secret_key_bytes();
    let hex_private_key = hex::encode(private_key);
    let public_key = kp.public_key_bytes();
    let key_identifier = KeyIdentifier::new(kp.get_key_derivator(), &public_key).to_str();
    match format {
        Format::Json => {
            let json = serde_json::to_string_pretty(&serde_json::json!({
                "private_key": hex_private_key,
                "controller_id": key_identifier,
                "peer_id": peer_id.to_string()
            }))
            .expect("JSON serialization possible");
            println!("{}", json);
        }
        Format::Yaml => {
            let yaml = serde_yaml::to_string(&serde_json::json!({
                "private_key": hex_private_key,
                "controller_id": key_identifier,
                "peer_id": peer_id.to_string()
            }))
            .expect("YAML serialization possible");
            println!("{}", yaml);
        }
    }
}

fn generate_ed25519() -> Ed25519KeyPair {
    Ed25519KeyPair::from_seed(&[])
}

fn generate_secp256k1() -> Secp256k1KeyPair {
    Secp256k1KeyPair::from_seed(&[])
}
