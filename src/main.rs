// Re-export the `config` module for convenience
mod helpers;

use helpers::tokenization;

fn main() {
    let token = tokenization::encode_token(String::from("abc"), "xyz", 3);
    println!("Encoded Token : {}", token);

    let decoded_token = tokenization::decode_token(&token, "xyz");
    println!("Decoded Payload : {:?}", decoded_token);
}
