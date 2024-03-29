use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore, Aes256Gcm, Key,
};
use base64::prelude::*;
use google::Certs;
use rmp_serde::{from_slice, to_vec};
use rocket::{fs::FileServer, launch, post, routes, Build, Rocket, State};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use std::env;

mod google;

#[derive(Debug, Serialize, Deserialize)]
struct Envelope {
    dek: String,
    iv: String,
    emails: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SealedEnvelope {
    data: String,
}

#[post("/seal", data = "<envelope>")]
fn seal(envelope: Json<Envelope>, kek: &State<Aes256Gcm>) -> Result<Json<SealedEnvelope>, Status> {
    let buf = to_vec::<Envelope>(&envelope).map_err(|_| Status::InternalServerError)?;
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = kek
        .encrypt(&nonce, buf.as_slice())
        .map_err(|_| Status::InternalServerError)?;
    let data =
        to_vec(&(nonce.to_vec(), ciphertext.to_vec())).map_err(|_| Status::InternalServerError)?;
    Ok(Json(SealedEnvelope {
        data: BASE64_URL_SAFE_NO_PAD.encode(&data),
    }))
}

#[post("/unseal", data = "<sealed_envelope>")]
fn unseal(
    sealed_envelope: Json<SealedEnvelope>,
    kek: &State<Aes256Gcm>,
    claims: google::Claims,
) -> Result<Json<Envelope>, Status> {
    let data = BASE64_URL_SAFE_NO_PAD
        .decode(&sealed_envelope.data)
        .map_err(|_| Status::Unauthorized)?;
    let (nonce, ciphertext): (Vec<u8>, Vec<u8>) =
        from_slice(&data).map_err(|_| Status::Unauthorized)?;
    let plaintext = kek
        .decrypt(nonce.as_slice().into(), ciphertext.as_slice())
        .map_err(|_| Status::Unauthorized)?;
    let envelope: Envelope = from_slice(&plaintext).map_err(|_| Status::Unauthorized)?;
    if !envelope.emails.contains(&claims.email) {
        return Err(Status::Unauthorized);
    }
    Ok(Json(envelope))
}

fn cipherly(base64_kek: &str, certs: Certs) -> Rocket<Build> {
    let bytes_kek = BASE64_URL_SAFE_NO_PAD.decode(base64_kek).unwrap();
    let kek = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&bytes_kek));

    rocket::build()
        .manage(kek)
        .manage(certs)
        .mount("/api", routes![seal, unseal])
        .mount("/", FileServer::from("./static"))
}

#[launch]
fn rocket() -> Rocket<Build> {
    env::set_var("ROCKET_PORT", env::var("PORT").unwrap_or("8000".into()));
    let kek = env::var("KEK").unwrap();
    let certs = google::fetch().unwrap();
    cipherly(&kek, certs)
}

#[cfg(test)]
mod tests {
    use super::cipherly;
    use crate::google::{parse, Claims};
    use jsonwebtoken::{encode, EncodingKey};
    use rocket::http::{Header, Status};
    use rocket::local::blocking::Client;

    const TEST_KEK: &str = "jRg36ErQ6FLcc7nZgngOpjJnJLGwA3xaMy0yx1pxJrI";
    const ALICE_ENVELOPE: &str = r#"{"dek":"gVwG8pMMMtdq6mS0OW19Kn7XwvdUcFJpkYN8cEnwnvs","iv":"9XfCog6Jp3MRgD71","emails":["alice@email.com"]}"#;

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

    fn client() -> Client {
        let certs = parse(include_str!("testdata/certs.json")).unwrap();
        Client::tracked(cipherly(TEST_KEK, certs)).expect("valid rocket instance")
    }

    #[test]
    fn post_seal_succeeds() {
        let client = client();
        let resp = client.post("/api/seal").body(ALICE_ENVELOPE).dispatch();
        assert!(resp.status() == Status::Ok);
        println!("{:?}", resp.into_string());
    }

    #[test]
    fn post_unseal_alice_succeeds() {
        let client = client();
        let resp = client
            .post("/api/unseal")
            .header(bearer("alice@email.com", "Alice"))
            .body(include_str!("testdata/alice.sealed"))
            .dispatch();
        assert!(resp.status() == Status::Ok);
    }

    #[test]
    fn post_unseal_eve_fails() {
        let client = client();
        let resp = client
            .post("/api/unseal")
            .header(bearer("eve@email.com", "Eve"))
            .body(include_str!("testdata/alice.sealed"))
            .dispatch();
        assert!(resp.status() == Status::Unauthorized);
    }

    #[test]
    fn post_unseal_no_auth_fails() {
        let client = client();
        let resp = client
            .post("/api/unseal")
            .body(include_str!("testdata/alice.sealed"))
            .dispatch();
        assert!(resp.status() == Status::Unauthorized);
    }

    #[test]
    fn seal_and_unseal_succeeds() {
        let client = client();
        let seal_resp = client.post("/api/seal").body(ALICE_ENVELOPE).dispatch();
        assert!(seal_resp.status() == Status::Ok);
        let body = seal_resp.into_string().unwrap();

        let unseal_resp = client
            .post("/api/unseal")
            .header(bearer("alice@email.com", "Alice"))
            .body(body)
            .dispatch();
        assert!(unseal_resp.status() == Status::Ok);
        assert_eq!(unseal_resp.into_string().unwrap(), ALICE_ENVELOPE);
    }
}
