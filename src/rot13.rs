use clap::Args;
use clap_stdin::MaybeStdin;

#[derive(Args, Debug)]
pub struct EnRot13Args {
    /// text rot rot13
    pub toencode: MaybeStdin<String>,
}

#[derive(Args, Debug)]
pub struct DeRot13Args {
    /// text rot rot13
    pub todecode: MaybeStdin<String>,
}

pub fn encode(args: EnRot13Args) {
    let toencode = args.toencode;
    let encoded = rot13(&toencode, 13);
    println!("{}", encoded);
}

pub fn decode(args: DeRot13Args) {
    let todecode = args.todecode;
    let decoded = rot13(&todecode, 13);
    println!("{}", decoded);
}

fn rot13(text: &str, amount: u8) -> String {
    text.chars()
        .map(|c| match c {
            'A'..='M' | 'a'..='m' => ((c as u8) + amount) as char,
            'N'..='Z' | 'n'..='z' => ((c as u8) - amount) as char,
            _ => c,
        })
        .collect()
}
