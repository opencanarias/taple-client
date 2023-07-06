use clap::{Parser, ValueEnum};
use taple_core::crypto::{Ed25519KeyPair, KeyGenerator, KeyMaterial, KeyPair, Secp256k1KeyPair};
use taple_core::identifier::{Derivable, KeyIdentifier};
use libp2p::core::PeerId;
use libp2p::identity::{ed25519::Keypair as EdKeyPair, secp256k1::SecretKey};

#[derive(Parser, Default, Debug)]
#[command(override_help = "
    MC generation utility for TAPLE nodes\n
\x1b[1m\x1b[4mUsage\x1b[0m: taple-keygen [OPTIONS] [MODE]\n
\x1b[1m\x1b[4mArguments\x1b[0m:
    [MODE]  Algorithm to use. Default to Ed25519 [possible values: ed25519, secp256k1]\n
\x1b[1m\x1b[4mOptions\x1b[0m:
    \x1b[1m-h, --help\x1b[0m           Print help information
    \x1b[1m-V, --version\x1b[0m        Print version information  
    ")]
#[clap(version)]
struct Args {
    /// Algorithm to use. Default to Ed25519
    #[clap(value_enum)]
    mode: Option<Algorithm>,
    /// Show only values
    #[clap(short, long)]
    suppress_info: bool,
}

#[derive(Parser, Clone, Debug, ValueEnum, Default)]
enum Algorithm {
    #[default]
    Ed25519,
    Secp256k1,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let (kp, alg_name, peer_id) = match args.mode.unwrap_or(Algorithm::Ed25519) {
        Algorithm::Ed25519 => {
            let keys = generate_ed25519();
            let peer_id = PeerId::from_public_key(
                &libp2p::identity::Keypair::Ed25519(
                    EdKeyPair::decode(&mut keys.to_bytes()).expect("Decode of Ed25519 possible"),
                )
                .public(),
            );
            let keys = KeyPair::Ed25519(keys);
            (keys, "ED25519", peer_id)
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
            (keys, "SECP256K1", peer_id)
        }
    };

    show_data(kp, alg_name, args.suppress_info, peer_id);
    Ok(())
}

fn show_data(kp: KeyPair, alg_name: &str, suppress_info: bool, peer_id: PeerId) {
    let private_key = kp.secret_key_bytes();
    let hex_private_key = hex::encode(private_key);
    let public_key = kp.public_key_bytes();
    let key_identifier = KeyIdentifier::new(kp.get_key_derivator(), &public_key).to_str();
    if !suppress_info {
        println!(
            "\x1b[1m\x1b[4m{}\x1b[0m: {}",
            format!("PRIVATE KEY {} (HEX)", alg_name),
            hex_private_key
        );
        println!(
            "\x1b[1m\x1b[4m{}\x1b[0m: {}",
            format!("CONTROLLER ID {}", alg_name),
            key_identifier
        );
        println!("\x1b[1m\x1b[4mPeerID\x1b[0m: {}", peer_id);
    } else {
        println!("{}", hex_private_key);
        println!("{}", key_identifier);
        println!("{}", peer_id);
    }
}

fn generate_ed25519() -> Ed25519KeyPair {
    Ed25519KeyPair::from_seed(&[])
}

fn generate_secp256k1() -> Secp256k1KeyPair {
    Secp256k1KeyPair::from_seed(&[])
}
