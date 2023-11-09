use urlencoding;

use clap::Args;



#[derive(Args, Debug)]
pub struct EnurlArgs {
    pub toencode: Vec<String>,

    /// Encode all characters
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Args, Debug)]
pub struct DeurlArgs {
    pub todecode: String
}


pub fn encode(args: EnurlArgs) {
    let toencode = args.toencode.join(" ");
    let all = args.all;
    
    let encoded = urlencoding::encode(&toencode);
    println!("{}", encoded);

    if all {
        println!("Printing testing lists...");
    } else {
        println!("Not printing testing lists...");
    }
}

pub fn decode(args: DeurlArgs) {
    let todecode: String = args.todecode;
    let decoded = urlencoding::decode(&todecode);
    println!("{}", decoded.unwrap());
}
