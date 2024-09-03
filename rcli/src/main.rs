extern crate core;

mod cli;
use clap::{Parser};
use zxcvbn::zxcvbn;
use rcli::{process_csv, process_genpass, Base64SubCommand, SubCommand};
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
                    println!("encode: {:?}", opts);
                }
                Base64SubCommand::Decode(opts) => {
                    println!("decode: {:?}", opts);
                }
            }
        }
    }
    Ok(())
}

