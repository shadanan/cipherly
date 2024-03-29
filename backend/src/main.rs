#[macro_use]
extern crate rocket;

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore, Aes256Gcm, Key,
};
use base64::prelude::*;
use rmp_serde::Serializer;
use rocket::{fs::FileServer, State};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use std::env;

mod google;

#[derive(Debug, Serialize, Deserialize)]
struct Envelope {
    dek: String,
    iv: String,
    authorized_users: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EncryptedEnvelope {
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EncodedEnvelope {
    header: String,
}

#[post("/encrypt", data = "<envelope>")]
fn encrypt(
    envelope: Json<Envelope>,
    kek: &State<Aes256Gcm>,
) -> Result<Json<EncodedEnvelope>, Status> {
    let mut buf = Vec::new();
    envelope
        .serialize(&mut Serializer::new(&mut buf))
        .map_err(|_| Status::InternalServerError)?;
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = kek
        .encrypt(&nonce, buf.as_slice())
        .map_err(|_| Status::InternalServerError)?;

    let mut header = Vec::new();
    let ee = EncryptedEnvelope {
        nonce: nonce.to_vec(),
        ciphertext: ciphertext.to_vec(),
    };
    ee.serialize(&mut Serializer::new(&mut header))
        .map_err(|_| Status::InternalServerError)?;
    let encoded_header = BASE64_URL_SAFE_NO_PAD.encode(&header);
    Ok(Json(EncodedEnvelope {
        header: encoded_header,
    }))
}

#[post("/decrypt", data = "<encoded_envelope>")]
fn decrypt(
    encoded_envelope: Json<EncodedEnvelope>,
    kek: &State<Aes256Gcm>,
    claims: google::Claims,
) -> Result<Json<Envelope>, Status> {
    println!("{:?}", encoded_envelope.header);
    let header = BASE64_URL_SAFE_NO_PAD
        .decode(&encoded_envelope.header)
        .map_err(|_| Status::Unauthorized)?;
    let ee: EncryptedEnvelope = rmp_serde::from_slice(&header).map_err(|_| Status::Unauthorized)?;
    let plaintext = kek
        .decrypt(ee.nonce.as_slice().into(), ee.ciphertext.as_slice())
        .map_err(|_| Status::Unauthorized)?;
    let envelope: Envelope = rmp_serde::from_slice(&plaintext).map_err(|_| Status::Unauthorized)?;
    if !envelope.authorized_users.contains(&claims.email) {
        return Err(Status::Unauthorized);
    }
    Ok(Json(envelope))
}

#[launch]
fn rocket() -> _ {
    env::set_var("ROCKET_PORT", env::var("PORT").unwrap_or("8000".into()));
    let base64_kek = env::var("KEK").unwrap();
    let bytes_kek = BASE64_URL_SAFE_NO_PAD.decode(base64_kek).unwrap();
    let kek = Key::<Aes256Gcm>::from_slice(&bytes_kek);
    let cipher = Aes256Gcm::new(kek);

    let mut builder = rocket::build()
        .manage(cipher)
        .mount("/api", routes![encrypt, decrypt])
        .mount("/", FileServer::from("./static"));

    if env::var("ROCKET_ENV").unwrap_or("unknown".into()) != "test" {
        let certs = google::fetch().unwrap();
        builder = builder.manage(certs);
    }

    builder
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use crate::google::tests::certs;
    use crate::google::Claims;
    use jsonwebtoken::{encode, EncodingKey};
    use rocket::http::{Header, Status};
    use rocket::local::blocking::Client;
    use std::env;

    const TEST_KEK: &str = "jRg36ErQ6FLcc7nZgngOpjJnJLGwA3xaMy0yx1pxJrI";

    fn bearer<'h>(email: &str, name: &str) -> Header<'h> {
        let encoding_key =
            EncodingKey::from_rsa_pem(include_str!("testdata/pk.pem").as_bytes()).unwrap();
        let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
        header.kid = Some("1".into());
        let claims = Claims {
            email: email.into(),
            name: name.into(),
            exp: 2524636800,
        };
        let token = encode(&header, &claims, &encoding_key).unwrap();
        Header::new("Authorization", format!("Bearer {}", token))
    }

    #[test]
    fn post_encrypt_succeeds() {
        env::set_var("KEK", TEST_KEK);
        env::set_var("ROCKET_ENV", "test");
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let resp = client
            .post("/api/encrypt")
            .body(
                r#"{
                    "dek":"gVwG8pMMMtdq6mS0OW19Kn7XwvdUcFJpkYN8cEnwnvs",
                    "iv":"9XfCog6Jp3MRgD71",
                    "authorized_users":["alice@email.com"]
                }"#,
            )
            .dispatch();
        assert!(resp.status() == Status::Ok);
        println!("{:?}", resp.into_string());
    }

    #[test]
    fn post_decrypt_alice_succeeds() {
        env::set_var("KEK", TEST_KEK);
        env::set_var("ROCKET_ENV", "test");
        let client = Client::tracked(rocket().manage(certs())).expect("valid rocket instance");
        let resp = client
            .post("/api/decrypt")
            .header(bearer("alice@email.com", "Alice"))
            .body(include_str!("testdata/alice.sealed"))
            .dispatch();
        assert!(resp.status() == Status::Ok);
    }

    #[test]
    fn post_decrypt_eve_fails() {
        env::set_var("KEK", TEST_KEK);
        env::set_var("ROCKET_ENV", "test");
        let client = Client::tracked(rocket().manage(certs())).expect("valid rocket instance");
        let resp = client
            .post("/api/decrypt")
            .header(bearer("eve@email.com", "Eve"))
            .body(include_str!("testdata/alice.sealed"))
            .dispatch();
        assert!(resp.status() == Status::Unauthorized);
    }
}
