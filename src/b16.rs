use clap::Args;
use clap_stdin::MaybeStdin;

#[derive(Args, Debug)]
pub struct EnB16Args {
    toencode: MaybeStdin<String>,
}

#[derive(Args, Debug)]
pub struct DeB16Args {
    todecode: MaybeStdin<String>,
}

pub fn encode(args: EnB16Args) {
    let toencode = args.toencode.to_string();
    let encoded = base16::encode_upper(&toencode);
    println!("{}", encoded);
}

pub fn decode(args: DeB16Args) {
    let decoded = base16::decode(&args.todecode.to_string()).unwrap();
    let decoded_str = String::from_utf8(decoded).expect("Our bytes should be valid utf8");
    println!("{}", decoded_str);
}
