use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

pub fn generate_keypair() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

pub fn encrypt_credentials(public_key: &RsaPublicKey, credentials: &Credentials) -> Vec<u8> {
    let data = serde_json::to_string(credentials).unwrap();
    public_key.encrypt(&mut rand::thread_rng(), Pkcs1v15Encrypt, data.as_bytes()).unwrap()
}

pub fn decrypt_credentials(private_key: &RsaPrivateKey, encrypted: &[u8]) -> Credentials {
    let decrypted = private_key.decrypt(Pkcs1v15Encrypt, encrypted).unwrap();
    serde_json::from_slice(&decrypted).unwrap()
}