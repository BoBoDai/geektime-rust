extern crate core;

mod cli;

use std::fs;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::{Parser};
use zxcvbn::zxcvbn;
use rcli::{process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, SubCommand, TextSubCommand, process_text_sign, get_reader, get_content, process_text_verify, process_text_key_generate};
use rcli::Opts;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let ret = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol)?;
            println!("{}", ret);
            let estimate = zxcvbn(&ret, &[]);
            eprintln!("Password strength: {}", estimate.score());
        }
        SubCommand::Base64(opts) => {
            match opts {
                Base64SubCommand::Encode(opts) => {
                    let mut  reader = get_reader(&opts.input)?;
                    let ret = process_encode(&mut reader, opts.format)?;
                    println!("{}", ret);
                }
                Base64SubCommand::Decode(opts) => {
                    let mut reader = get_reader(&opts.input)?;
                    let ret = process_decode(&mut reader, opts.format)?;
                    println!("{}", ret);
                }
            }
        }
        SubCommand::Text(cmd) => match cmd {
            TextSubCommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = process_text_sign(&mut reader, &key, opts.format)?;
                let encoded = URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encoded);
            }
            TextSubCommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified = process_text_verify(&mut reader, &key, &decoded, opts.format)?;
                if verified {
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_key_generate(opts.format)?;
                for (k, v) in key {
                    fs::write(opts.output_path.join(k), v)?;
                }
            }
        }
    }
    Ok(())
}

