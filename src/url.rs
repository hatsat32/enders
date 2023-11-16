use clap::Args;

#[derive(Args, Debug)]
pub struct EnurlArgs {
    pub toencode: Vec<String>,
}

#[derive(Args, Debug)]
pub struct DeurlArgs {
    pub todecode: String,
}

pub fn encode(args: EnurlArgs) {
    let toencode = args.toencode.join(" ");

    let encoded = urlencoding::encode(&toencode);
    println!("{}", encoded);
}

pub fn decode(args: DeurlArgs) {
    let todecode: String = args.todecode;
    let decoded = urlencoding::decode(&todecode);
    println!("{}", decoded.unwrap());
}
