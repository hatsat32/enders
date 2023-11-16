use clap::Args;

#[derive(Args, Debug)]
pub struct EnRot13Args {
    /// text rot rot13
    pub toencode: Vec<String>,
}

#[derive(Args, Debug)]
pub struct DeRot13Args {
    /// text rot rot13
    pub todecode: Vec<String>,
}

pub fn encode(args: EnRot13Args) {
    let toencode: String = args.toencode.join(" ");
    let encoded = rot13(&toencode, 13);
    println!("{}", encoded);
}

pub fn decode(args: DeRot13Args) {
    let todecode: String = args.todecode.join(" ");
    let decoded = rot13(&todecode, 13);
    println!("{}", decoded);
}

fn rot13(text: &str, amount: u8) -> String {
    text.chars().map(|c| {
        match c {
            'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + amount) as char,
            'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - amount) as char,
            _ => c
        }
    }).collect()
}
