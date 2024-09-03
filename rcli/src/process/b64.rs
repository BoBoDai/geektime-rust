use std::string::String;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use crate::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let encoded = STANDARD.encode(input);
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let decoded = STANDARD.decode(input)?;
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}
