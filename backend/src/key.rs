#[cfg(test)]
mod tests {
    use aes_gcm::{
        aead::{Aead, KeyInit, OsRng},
        Aes256Gcm,
        Key, // Or `Aes128Gcm`
        Nonce,
    };
    use base64::prelude::*;

    #[test]
    fn generate_key() {
        let key = Aes256Gcm::generate_key(OsRng);
        // base64 encode
        println!("Master Key: {}", BASE64_STANDARD.encode(key));
    }

    #[test]
    fn whatever() {
        let base64_test_kek = "0MmC28nuauYqXE7mZ5JL08ydOkmk+A5q3Y0tsn5+izg=";
        let bytes_test_kek = BASE64_STANDARD.decode(base64_test_kek).unwrap();
        let test_kek = Key::<Aes256Gcm>::from_slice(&bytes_test_kek);
        let cipher = Aes256Gcm::new(&test_kek);
        let nonce = Nonce::from_slice(&[0u8; 12]);
        let plaintext = b"Hello, world!";
        let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).unwrap();
        let decrypted = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
        assert_eq!(plaintext.as_slice(), decrypted);
    }
}
