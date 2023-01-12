mod structures;

use crate::structures::key_pair::{KeyGeneration, KeyPair};

fn main() {
    //generate a key pair
    let key_pair = KeyPair::generate_key_pair();

    //print
    println!("Public Key: {}", key_pair.public_key.clone());
    println!("Private Key: {}", key_pair.private_key.clone());
    //
    // //encrypt
    // let message = String::from("Hello World!");
    // let ciphertext = structures::rsa_encryption::RSAEncryption::encrypt(message.clone(), key_pair.public_key.clone());
    //
    // //print
    // println!("Ciphertext: {}", ciphertext.clone());
    //
    // //decrypt
    // let plaintext = structures::rsa_encryption::RSAEncryption::decrypt(ciphertext.clone(), key_pair.private_key.clone());
    //
    // //print
    // println!("Plaintext: {}", plaintext.clone());



}