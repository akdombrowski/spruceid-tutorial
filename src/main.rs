// use std::ffi::OsStr;
use std::ffi::OsString;
use std::ops::Deref;

use clap::builder::ValueParser;
use clap::{Args, Parser, Subcommand, ValueHint};

#[derive(Parser)]
#[command(name = "vc-cli")]
#[command(version)]
#[command(
    about = "DIDKit-based VC generator and verifier",
    long_about = "A Verification Credential generator and verifier using SpruceID's DIDKit"
)]
#[command(bin_name = "vc-cli")]
#[command(display_name = "vc-cli")]
#[command(next_line_help = true)]
#[command(args_conflicts_with_subcommands = true)]
struct Cli {
    /// path to a .json file containing the unsigned VC
    #[arg(value_name = "UNSIGNED_VC")]
    #[arg(value_hint = ValueHint::FilePath)]
    #[arg(value_parser = ValueParser::os_string())]
    #[arg(requires = "key")]
    cred: OsString,

    /// path to the .jwk file containing the signing key
    #[arg(value_name = "PUBLIC_KEY")]
    #[arg(value_hint = ValueHint::FilePath)]
    #[arg(requires = "out_file")]
    key: OsString,

    /// path to the file to send the output
    #[arg(value_name = "OUT_FILE")]
    #[arg(value_hint = ValueHint::FilePath)]
    out_file: OsString,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[command(flatten_help = true)]
enum Commands {
    /// Generates a signed VC (Verifiable Credential)
    Generate(GenerateArgs),

    /// Validates a signed VC (Verifiable Credential)
    Verify(VerifyArgs),
}

#[derive(Args)]
#[command(next_line_help = true)]
#[command(args_conflicts_with_subcommands = true)]
struct GenerateArgs {
    /// path to a .json file containing the unsigned VC
    #[arg(value_name = "UNSIGNED_VC")]
    #[arg(value_hint = ValueHint::FilePath)]
    cred: OsString,

    /// path to the .jwk file containing the key to use for verification
    #[arg(value_name = "PRIVATE_KEY")]
    #[arg(value_hint = ValueHint::FilePath)]
    key: OsString,

    // TODO: does
    /// path to the file to send the output
    #[arg(value_name = "OUT_FILE")]
    #[arg(value_hint = ValueHint::FilePath)]
    out_file: OsString,
}

#[derive(Args)]
#[command(next_line_help = true)]
struct VerifyArgs {
    /// path to a .json file containing the signed VC
    #[arg(value_name = "SIGNED_VC")]
    cred: OsString,

    /// path to the .jwk file containing the key to use for verification
    #[arg(value_name = "PRIVATE_KEY")]
    key: OsString,

    // TODO: does
    /// path to the file to send the output
    #[arg(value_name = "OUT_FILE")]
    out_file: OsString,
}

fn main() {
    let args = Cli::parse();
    let cred = args.cred.deref();
    let key = args.key.deref();
    let out_file = args.out_file.deref();

    if cred.to_str().is_some() {
        println!("Value for cred: ");
        println!("{:?}", cred);
    }
    if key.to_str().is_some() {
        println!("Value for key: ");
        println!("{:?}", key);
    }
    if out_file.to_str().is_some() {
        println!("Value for out_file: ");
        println!("{:?}", out_file);
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &args.command {
        Some(Commands::Generate(generate_args)) => {
            let GenerateArgs {
                cred,
                key,
                out_file,
            } = generate_args;

            if cred.to_str().is_some() {
                println!("Printing cred info input...");
                println!("{:?}", cred);
            } else {
                println!("Not printing cred info input...");
            }

            if key.to_str().is_some() {
                println!("Printing key input...");
                println!("{:?}", key);
            } else {
                println!("where's the key?!?...");
            }

            if out_file.to_str().is_some() {
                println!("Printing output file...");
            } else {
                println!("Not printing output file...");
            }
        }

        Some(Commands::Verify(verify_args)) => {
            let VerifyArgs {
                cred,
                key,
                out_file,
            } = verify_args;

            if cred.to_str().is_some() {
                println!("Printing cred info input...");
            } else {
                println!("Not printing cred info input...");
            }

            if key.to_str().is_some() {
                println!("Printing key input...");
                println!("{:?}", key);
            } else {
                println!("where's the key?!?...");
            }

            if out_file.to_str().is_some() {
                println!("Printing output file...");
            } else {
                println!("Not printing output file...");
            }
        }

        None => {}
    }

    // match args.command {
    //     Some(Commands::Generate(generate_args)) => {
    //         let GenerateArgs {
    //             cred,
    //             key,
    //             out_file,
    //         } = generate_args;

    //         if cred.is_some() {
    //             println!("Printing cred info input...");
    //             println!("{:?}", cred);
    //         } else {
    //             println!("Not printing cred info input...");
    //         }

    //         if key.is_some() {
    //             println!("Printing key input...");
    //             println!("{:?}", key);
    //         } else {
    //             println!("where's the key?!?...");
    //         }

    //         if { out_file }.is_some() {
    //             println!("Printing output file...");
    //         } else {
    //             println!("Not printing output file...");
    //         }

    //         cred.as_deref().unwrap_or_else(|| OsStr::new(""));
    //     }

    //     Some(Commands::Verify(verify_args)) => {
    //         let VerifyArgs {
    //             cred,
    //             key,
    //             out_file,
    //         } = verify_args;
    //         if cred.is_some() {
    //             println!("Printing cred info input...");
    //         } else {
    //             println!("Not printing cred info input...");
    //         }

    //         if key.is_some() {
    //             println!("Printing key input...");
    //             println!("{:?}", key);
    //         } else {
    //             println!("where's the key?!?...");
    //         }

    //         if out_file.is_some() {
    //             println!("Printing output file...");
    //         } else {
    //             println!("Not printing output file...");
    //         }

    //         cred.as_deref().unwrap_or_else(|| OsStr::new(""));
    //     }

    //     None => {}
    // }

    // Continued program logic goes here...
}
