#[cfg(test)]
mod tests {
    use aes_gcm::{
        aead::{KeyInit, OsRng},
        Aes256Gcm,
    };
    use base64::prelude::*;

    #[test]
    fn generate_key() {
        let key = Aes256Gcm::generate_key(OsRng);
        // base64 encode
        println!("Master Key: {}", BASE64_STANDARD.encode(key));
    }
}
