use crate::structures::key_pair::KeyPair;

trait RSAEncryption {

    //encrypt
    fn encrypt(message: String, public_key: String) -> String;

    //decrypt
    fn decrypt(ciphertext: String, private_key: String) -> String;

}

//implement
impl RSAEncryption for KeyPair {

    fn encrypt(message: String, public_key: String) -> String {
        // Encryption c = (msg ^ e) % n

        //split the public key
        let public_key_chars: Vec<&str> = public_key.split(",").collect();

        "Hello World!".to_string()
    }

    fn decrypt(ciphertext: String, private_key: String) -> String {
        // Decryption m = (c ^ d) % n

        //split the private key
        let private_key_chars: Vec<&str> = private_key.split(",").collect();

        "Hello World!".to_string()
    }

}
