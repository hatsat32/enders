use clap_stdin::MaybeStdin;
use data_encoding::HEXLOWER_PERMISSIVE;

use clap::Args;

#[derive(Args, Debug)]
pub struct EnHexArgs {
    toencode: MaybeStdin<String>,
}

#[derive(Args, Debug)]
pub struct DeHexArgs {
    todecode: MaybeStdin<String>,
}

pub fn encode(args: EnHexArgs) {
    let encoded = HEXLOWER_PERMISSIVE.encode(args.toencode.to_string().as_bytes());
    println!("{}", encoded);
}

pub fn decode(args: DeHexArgs) {
    let decoded: Vec<u8> = HEXLOWER_PERMISSIVE
        .decode(args.todecode.as_bytes())
        .unwrap();
    let decoded_str = String::from_utf8(decoded).expect("Our bytes should be valid utf8");
    println!("{}", decoded_str);
}
