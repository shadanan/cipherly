use aes_gcm::{
    aead::{KeyInit, OsRng},
    Aes256Gcm,
};
use base64::prelude::*;

fn main() {
    let key = Aes256Gcm::generate_key(OsRng);
    println!("New KEK: {}", BASE64_URL_SAFE_NO_PAD.encode(key));
}
