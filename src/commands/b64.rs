use base64::{engine::general_purpose, Engine as _};



pub fn encode (toencode: String) {
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(toencode);
    println!("{}", encoded);
}


pub fn decode (todecode: String) {
    let decoded = general_purpose::STANDARD_NO_PAD.decode(todecode).unwrap();
    let string = String::from_utf8(decoded).expect("Our bytes should be valid utf8");
    println!("{}", string);
}
