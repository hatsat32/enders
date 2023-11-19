use clap::Args;
use clap_stdin::MaybeStdin;

#[derive(Args, Debug)]
pub struct EnurlArgs {
    pub toencode: MaybeStdin<String>,
}

#[derive(Args, Debug)]
pub struct DeurlArgs {
    pub todecode: MaybeStdin<String>,
}

pub fn encode(args: EnurlArgs) {
    let toencode = args.toencode;
    let encoded = urlencoding::encode(&toencode);
    println!("{}", encoded)
}

pub fn decode(args: DeurlArgs) {
    let todecode = args.todecode;
    let decoded = urlencoding::decode(&todecode).unwrap();
    println!("{}", decoded)
}
