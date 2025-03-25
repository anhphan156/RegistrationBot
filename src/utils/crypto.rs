use ed25519_dalek::{Signature, VerifyingKey, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};

pub struct Crypto;

impl Crypto {
    pub fn verify_key(message: &str, signature: &str)-> bool {
        let public_key = match std::env::var("PUBLIC_KEY") {
            Ok(key) => key,
            Err(_) => return false
        };
        let public_key : [u8; PUBLIC_KEY_LENGTH] = hex::decode(public_key)
            .expect("Failed to decode public key")
            .try_into()
            .expect("Invalid public key byte length");

        if let Ok(public_key) = VerifyingKey::from_bytes(&public_key) {
            let signature : [u8; SIGNATURE_LENGTH] = hex::decode(signature)
                .expect("Failed to decode signature")
                .try_into()
                .expect("Invalid signature byte length");
            let signature = Signature::from_bytes(&signature);

            match public_key.verify_strict(message.as_bytes(), &signature) {
                Ok(_) => return true,
                Err(e) => {
                    println!("Error: {}",e); 
                    return false;
                }
            }
        }

        false
    }
}
