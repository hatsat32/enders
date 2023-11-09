use urlencoding;

pub fn encode(toencode: String, all: bool) {
    let encoded = urlencoding::encode(&toencode);
    println!("{}", encoded);

    if all {
        println!("Printing testing lists...");
    } else {
        println!("Not printing testing lists...");
    }
}

pub fn decode(todecode: String) {
    let decoded = urlencoding::decode(&todecode);
    println!("{}", decoded.unwrap());
}
