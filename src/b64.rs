use base64::{engine::general_purpose, Engine as _};
use clap::Args;
use clap_stdin::MaybeStdin;

#[derive(Args, Debug)]
pub struct Enb64Args {
    toencode: MaybeStdin<String>,
}

#[derive(Args, Debug)]
pub struct Deb64Args {
    todecode: MaybeStdin<String>,
}

pub fn encode(enb64args: Enb64Args) {
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(enb64args.toencode.to_string());
    println!("{}", encoded);
}

pub fn decode(args: Deb64Args) {
    let decoded = general_purpose::STANDARD_NO_PAD
        .decode(args.todecode.to_string())
        .unwrap();
    let decoded_str = String::from_utf8(decoded).expect("Our bytes should be valid utf8");
    println!("{}", decoded_str);
}
