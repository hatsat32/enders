use clap_stdin::MaybeStdin;
use data_encoding::BASE32_NOPAD;

use clap::Args;

#[derive(Args, Debug)]
pub struct Enb32Args {
    toencode: MaybeStdin<String>,
}

#[derive(Args, Debug)]
pub struct Deb32Args {
    todecode: MaybeStdin<String>,
}

pub fn encode(args: Enb32Args) {
    // let encoded: String = general_purpose::STANDARD_NO_PAD.encode(args.toencode.join(" "));
    let encoded = BASE32_NOPAD.encode(args.toencode.as_bytes());
    println!("{}", encoded);
}

pub fn decode(args: Deb32Args) {
    let decoded: Vec<u8> = BASE32_NOPAD.decode(args.todecode.as_bytes()).unwrap();
    let decoded_str = String::from_utf8(decoded).expect("Our bytes should be valid utf8");
    println!("{}", decoded_str);
}
