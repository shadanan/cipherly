use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use rocket::request::FromRequest;
use rocket::State;
use rocket::{http::Status, request::Outcome, Request};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Key {
    pub kid: String,
    pub alg: String,
    pub n: String,
    pub e: String,
    pub kty: String,
    pub r#use: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Certs {
    pub keys: Vec<Key>,
}

impl Certs {
    pub fn get(&self, kid: &str) -> Result<DecodingKey, Box<dyn Error>> {
        for key in &self.keys {
            if key.kid == *kid {
                return Ok(DecodingKey::from_rsa_components(&key.n, &key.e)?);
            }
        }
        Err("missing".into())
    }
}

pub fn fetch() -> Result<Certs, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get("https://www.googleapis.com/oauth2/v3/certs")
        .send()?
        .text()?;
    let certs: Certs = serde_json::from_str(&resp)?;
    Ok(certs)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub name: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let certs = request.guard::<&State<Certs>>().await.unwrap();
        let Some(auth_header) = request.headers().get_one("Authorization") else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        let parts: Vec<&str> = auth_header.split(' ').collect();
        if parts.len() != 2 || parts[0] != "Bearer" {
            return Outcome::Error((Status::Unauthorized, ()));
        }
        let bearer = parts[1];
        let Ok(header) = jsonwebtoken::decode_header(bearer) else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        let Some(kid) = header.kid else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        let Ok(key) = certs.get(&kid) else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_audience(&[
            "981002175662-g8jr2n89bptsn8n9ds1fn5edfheojr7i.apps.googleusercontent.com",
        ]);
        validation.set_issuer(&["https://accounts.google.com"]);
        let Ok(token) = jsonwebtoken::decode::<Claims>(bearer, &key, &validation) else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        Outcome::Success(Claims {
            sub: token.claims.sub,
            email: token.claims.email,
            name: token.claims.name,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::google::fetch;
    use crate::google::{Certs, Key};

    #[test]
    fn fetch_succeeds() {
        let result = fetch();
        assert!(result.is_ok());
    }

    fn certs() -> Certs {
        Certs {
            keys: vec![
                Key {
                    kid: "09bcf8028e06537d4d3ae4d84f5c5babcf2c0f0a".to_string(), 
                    alg: "RS256".to_string(), 
                    n: "vdtZ3cfuh44JlWkJRu-3yddVp58zxSHwsWiW_jpaXgpebo0an7qY2IEs3D7kC186Bwi0T7Km9mUcDbxod89IbtZuQQuhxlgaXB-qX9GokNLdqg69rUaealXGrCdKOQ-rOBlNNGn3M4KywEC98KyQAKXe7prs7yGqI_434rrULaE7ZFmLAzsYNoZ_8l53SGDiRaUrZkhxXOEhlv1nolgYGIH2lkhEZ5BlU53BfzwjO-bLeMwxJIZxSIOy8EBIMLP7eVu6AIkAr9MaDPJqeF7n7Cn8yv_qmy51bV-INRS-HKRVriSoUxhQQTbvDYYvJzHGYu_ciJ4oRYKkDEwxXztUew".to_string(), 
                    e: "AQAB".to_string(), 
                    kty: "RSA".to_string(), 
                    r#use: "sig".to_string() 
                },
                Key {
                    kid: "adf5e710edfebecbefa9a61495654d03c0b8edf8".to_string(), 
                    alg: "RS256".to_string(), 
                    n: "y48N6JB-AKq1-Rv4SkwBADU-hp4zXHU-NcCUwxD-aS9vr4EoT9qrjoJ-YmkaEpq9Bmu1yXZZK_h_9QS3xEsO8Rc_WSvIQCJtIaDQz8hxk4lUjUQjMB4Zf9vdTmf8KdktI9tCYCbuSbLC6TegjDM9kbl9CNs3m9wSVeO_5JXJQC0Jr-Oj7Gz9stXm0Co3f7RCxrD08kLelXaAglrd5TeGjZMyViC4cw1gPaj0Cj6knDn8UlzR_WuBpzs_ies5BrbzX-yht0WfnhXpdpiGNMbpKQD04MmPdMCYq8ENF7q5_Ok7dPsVj1vHA6vFGnf7qE3smD157szsnzn0NeXIbRMnuQ".to_string(), 
                    e: "AQAB".to_string(), 
                    kty: "RSA".to_string(), 
                    r#use: "sig".to_string() 
                }
            ]
        }
    }

    #[test]
    fn get_returns_error_if_kid_is_missing() {
        let certs = certs();
        assert!(certs.get("missing").is_err())
    }

    #[test]
    fn get_returns_key_if_kid_exists() {
        let certs = certs();
        assert!(certs
            .get("adf5e710edfebecbefa9a61495654d03c0b8edf8")
            .is_ok())
    }
}
