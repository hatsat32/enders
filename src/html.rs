use clap::Args;
use html_escape;


#[derive(Args, Debug)]
pub struct EnHtmlArgs {
    toencode: Vec<String>,
}

#[derive(Args, Debug)]
pub struct DeHtmlArgs {
    todecode: String,
}

pub fn encode(args: EnHtmlArgs) {
    let toencode = args.toencode.join(" ");
    let encoded = html_escape::encode_text(&toencode);
    println!("{}", encoded);
}

pub fn decode(args: DeHtmlArgs) {
    let decoded = html_escape::decode_html_entities(&args.todecode);
    println!("{}", decoded);
}
