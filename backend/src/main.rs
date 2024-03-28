#[macro_use]
extern crate rocket;

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore, Aes256Gcm, Key,
};
use base64::prelude::*;
use rmp_serde::Serializer;
use rocket::serde::json::Json;
use rocket::{fs::FileServer, State};
use serde::{Deserialize, Serialize};
use std::env;

mod google;

#[derive(Serialize)]
struct Time {
    time: String,
}

#[get("/time")]
fn time(claims: google::Claims) -> Json<Time> {
    println!("{:?}", claims);
    Json(Time {
        time: chrono::Utc::now().to_rfc3339(),
    })
}

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
fn encrypt(envelope: Json<Envelope>, kek: &State<Aes256Gcm>) -> Json<EncodedEnvelope> {
    let mut buf = Vec::new();
    envelope.serialize(&mut Serializer::new(&mut buf)).unwrap();
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = kek.encrypt(&nonce, buf.as_slice()).unwrap();

    let mut header = Vec::new();
    let ee = EncryptedEnvelope {
        nonce: nonce.to_vec(),
        ciphertext: ciphertext.to_vec(),
    };
    ee.serialize(&mut Serializer::new(&mut header)).unwrap();
    let encoded_header = BASE64_URL_SAFE_NO_PAD.encode(&header);
    Json(EncodedEnvelope {
        header: encoded_header,
    })
}

#[post("/decrypt", data = "<encoded_envelope>")]
fn decrypt(
    encoded_envelope: Json<EncodedEnvelope>,
    kek: &State<Aes256Gcm>,
    claims: google::Claims,
) -> Json<Envelope> {
    println!("{:?}", encoded_envelope.header);
    let header = BASE64_URL_SAFE_NO_PAD
        .decode(&encoded_envelope.header)
        .unwrap();
    let ee: EncryptedEnvelope = rmp_serde::from_slice(&header).unwrap();
    let plaintext = kek
        .decrypt(ee.nonce.as_slice().into(), ee.ciphertext.as_slice())
        .unwrap();
    let envelope: Envelope = rmp_serde::from_slice(&plaintext).unwrap();
    assert!(envelope.authorized_users.contains(&claims.email));
    Json(envelope)
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
        .mount("/api", routes![time, encrypt, decrypt])
        .mount("/", FileServer::from("./static"));

    if env::var("ROCKET_ENV").unwrap_or("unknown".into()) != "test" {
        let certs = google::fetch().unwrap();
        builder = builder.manage(certs);
    }

    builder
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::google::{Certs, Key};

    use super::rocket;
    use rocket::http::{Header, Status};
    use rocket::local::blocking::Client;

    const TEST_KEK: &str = "jRg36ErQ6FLcc7nZgngOpjJnJLGwA3xaMy0yx1pxJrI";
    const TEST_PUBLIC_KEY: &str = "m6KbCqXkrPIoBUNfNshI90G6HijNVKeFytm3ATzfzMfFgqDUx9lHb86BZ1cvFFJ6CEUl_0-yUHdnZO8pa1ENUIT4FCYHkPTvfRkkl03ZxmWzSCpmIqLerBoyxgE-z2DwNPnH3KOwtnipxREGDeTRsoP4SZ8IUcbFXi10a58l3AI-toMqFEnuMTmcicEXA0k9-3RjFId-n5OJvqn1ucNIk04ZvvH4fkl1_rF98xou00MRniix_p6S6Os-bn875NHpdoEEQ0IRClg9GZp_prgRFmG8xYaYqJM7VlVz7AdSWDsP62G4JcuK1TUzARWPTv-5wW_Pb2CKDclS057WXyUd3w";
    const BEARER: &str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IjEifQ.eyJzdWIiOiIxIiwiZW1haWwiOiJ1c2VyQGVtYWlsLmNvbSIsIm5hbWUiOiJVc2VyIEJvYiIsImV4cCI6MjUyNDYzNjgwMH0.gj6rve7hYcfo5RqaRai1cdKAy5ao3fIDDT4An8moAHLlIPN_q0aoYPSmRl_rpp-a8uxSeAdECtHayqJN51VaaEYV-O1-g7gHEuWE9lRcv-ITF8WjAmkLwerQhsc6dJ1_NK9_hU5MZOGp1WzKcBqcIKK6P5OadzuXx2kTSor7xvT16-n7-wnfMni_fjNHCLbKmiQQjrzJLwCidG0MiBOa8Nft7d3LbVCeQPEKsos0p-8kkj-64O9CTK7RxpkgIGPbwig69TkZYtPmeKUm2O1VnQMICZQldTXcRcQ7UZARO4PxICwrxofljJ_p2DBTe9fgefUaKTBA0eugzR3p3y0A7A";

    fn test_certs() -> Certs {
        Certs {
            keys: vec![Key {
                alg: "RS256".into(),
                kid: "1".into(),
                n: TEST_PUBLIC_KEY.into(),
                e: "AQAB".into(),
                kty: "RSA".into(),
                r#use: "sig".into(),
            }],
        }
    }

    #[test]
    fn post_encrypt_succeeds() {
        env::set_var("KEK", TEST_KEK);
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let resp = client
            .post("/api/encrypt")
            .body(
                r#"{
                    "dek":"gVwG8pMMMtdq6mS0OW19Kn7XwvdUcFJpkYN8cEnwnvs",
                    "iv":"9XfCog6Jp3MRgD71",
                    "authorized_users":["user@email.com"]
                }"#,
            )
            .dispatch();
        assert!(resp.status() == Status::Ok);
    }

    #[test]
    fn post_decrypt_succeeds() {
        env::set_var("KEK", TEST_KEK);
        let client = Client::tracked(rocket().manage(test_certs())).expect("valid rocket instance");
        let resp = client.post("/api/decrypt").header(Header::new("Authorization", BEARER)).body(r#"{"header":"kpzM_8zbzNVwHszfSszmIczcFj_cAF_MgRjMvTvM08zMaDtfzM7MrcztzKhSzKhUzLbM6MywBgXM8szlMylXzMDMm8zRzMfM3x_M5RTMoARYzLU_cnE3IALM31UPzPDMtMzeDQHMlMzKzJrMn8yzzKXMlgfMygYCzNnMigsuJV_M2WUrH8zRS8ykzKw_zKUPTczbDRV0zPUwaxbMgwXMkkwAzNc"}"#).dispatch();
        assert!(resp.status() == Status::Ok);
    }

    #[test]
    fn get_time_succeeds() {
        env::set_var("ROCKET_ENV", "test");
        let client = Client::tracked(rocket().manage(test_certs())).expect("valid rocket instance");
        let resp = client
            .get("/api/time")
            .header(Header::new("Authorization", BEARER))
            .dispatch();
        assert!(resp.status() == Status::Ok);
    }
}
