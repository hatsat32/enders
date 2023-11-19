use clap::Args;
use clap_stdin::MaybeStdin;

#[derive(Args, Debug)]
pub struct EnHtmlArgs {
    toencode: MaybeStdin<String>,
}

#[derive(Args, Debug)]
pub struct DeHtmlArgs {
    todecode: MaybeStdin<String>,
}

pub fn encode(args: EnHtmlArgs) {
    let toencode = args.toencode.to_string();
    let encoded = html_escape::encode_text(&toencode);
    println!("{}", encoded);
}

pub fn decode(args: DeHtmlArgs) {
    let toencode = args.todecode.to_string();
    let decoded = html_escape::decode_html_entities(&toencode);
    println!("{}", decoded);
}
