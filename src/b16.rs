use clap::Args;
use base16;


#[derive(Args, Debug)]
pub struct EnB16Args {
    toencode: Vec<String>,
}

#[derive(Args, Debug)]
pub struct DeB16Args {
    todecode: String,
}

pub fn encode(args: EnB16Args) {
    let toencode = args.toencode.join(" ");
    let encoded = base16::encode_upper(&toencode);
    println!("{}", encoded);
}

pub fn decode(args: DeB16Args) {
    let decoded = base16::decode(&args.todecode).unwrap();
    let decoded_str = String::from_utf8(decoded).expect("Our bytes should be valid utf8");
    println!("{}", decoded_str);
}

