use clap::Args;

#[derive(Args, Debug)]
pub struct EnRot13Args {
    pub toencode: Vec<String>,
}

#[derive(Args, Debug)]
pub struct DeRot13Args {
    pub todecode: Vec<String>,
}

pub fn encode(args: EnRot13Args) {
    let encoded: String = args.toencode.join(" ");
    println!("{}", encoded);
}

pub fn decode(args: DeRot13Args) {
    let decoded: String = args.todecode.join(" ");
    println!("{}", decoded);
}
