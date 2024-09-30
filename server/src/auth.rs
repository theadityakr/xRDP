use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
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
    public_key.encrypt(&mut rand::thread_rng(), PaddingScheme::new_pkcs1v15_encrypt(), data.as_bytes()).unwrap()
}

pub fn decrypt_credentials(private_key: &RsaPrivateKey, encrypted: &[u8]) -> Credentials {
    let decrypted = private_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), encrypted).unwrap();
    serde_json::from_slice(&decrypted).unwrap()
}  