use clap::Args;
use base64::{engine::general_purpose, Engine as _};


#[derive(Args, Debug)]
pub struct Enb64Args {
    toencode: Vec<String>
}

#[derive(Args, Debug)]
pub struct Deb64Args {
    todecode: String,
}

pub fn encode(enb64args: Enb64Args) {
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(enb64args.toencode.join(" "));
    println!("{}", encoded);
}

pub fn decode(args: Deb64Args) {
    let todecode = args.todecode;
    let decoded = general_purpose::STANDARD_NO_PAD.decode(todecode).unwrap();
    let string = String::from_utf8(decoded).expect("Our bytes should be valid utf8");
    println!("{}", string);
}
