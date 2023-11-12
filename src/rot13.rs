use clap::Args;

#[derive(Args, Debug)]
pub struct EnRot13Args {
    /// text rot rot13
    pub toencode: Vec<String>,

    #[arg(short, long, default_value_t = 13)]
    amount: u8,
}

#[derive(Args, Debug)]
pub struct DeRot13Args {
    /// text rot rot13
    pub todecode: Vec<String>,

    #[arg(short, long, default_value_t = 13)]
    amount: u8,
}

pub fn encode(args: EnRot13Args) {
    let toencode: String = args.toencode.join(" ");
    let encoded = rot13(&toencode, args.amount);
    println!("{}", encoded);
}

pub fn decode(args: DeRot13Args) {
    let todecode: String = args.todecode.join(" ");
    let decoded = rot13(&todecode, args.amount);
    println!("{}", decoded);
}

fn rot13(text: &str, amount: u8) -> String {
    // let amount = amount.unwrap_or(13);
    text.chars().map(|c| {
        match c {
            'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + amount) as char,
            'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - amount) as char,
            _ => c
        }
    }).collect()
}
