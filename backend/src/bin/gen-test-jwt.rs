use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::error::Error;

const TEST_RSA_PRIVATE_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCbopsKpeSs8igF
Q182yEj3QboeKM1Up4XK2bcBPN/Mx8WCoNTH2UdvzoFnVy8UUnoIRSX/T7JQd2dk
7ylrUQ1QhPgUJgeQ9O99GSSXTdnGZbNIKmYiot6sGjLGAT7PYPA0+cfco7C2eKnF
EQYN5NGyg/hJnwhRxsVeLXRrnyXcAj62gyoUSe4xOZyJwRcDST37dGMUh36fk4m+
qfW5w0iTThm+8fh+SXX+sX3zGi7TQxGeKLH+npLo6z5ufzvk0el2gQRDQhEKWD0Z
mn+muBEWYbzFhpiokztWVXPsB1JYOw/rYbgly4rVNTMBFY9O/7nBb89vYIoNyVLT
ntZfJR3fAgMBAAECggEALqZiyqJHFP/QtyMmmwkVtedhnkCkq0c7dz4i+KAChSMs
NKd4oB6kpwLDDvK3DeRa57eyWADPdgcUWtDFHYHB9+yjX4a2peCFGH481Y1kRW+e
eXcb/i3LiNKhfXBKH2bzGS31llEKWud4G7WqlueLY3LjLU4Z9EkcR8pgJxIhCvy5
I8u2f0edcCeEldT+SQgr7JnT4EUzNDXQcipTMTcylommVs3UIdKk80d72t07YbiP
VSsNa/eh9OgLuz4spbo648KrHjfwdxzOMrBlz580GLBIKqgihu460plMmvH+TdTr
PA+/VTlKCX8Sf1xkUj+aOOWZS7wZ5DtCdIOApn5XQQKBgQDaRDQ2uglbHQ3gAC3N
IyeT09z7W4/5iSlSZGcIps3o1SUyifn+BC9CLex2BbqY9BeJwEvkxrLOIXgY+1Vy
iVPcxGqo0F7+VAdxwI8YW2gJ/bTnKKt+7mIulZ+Xw3fjS505Yn5TJw8MEkq6j1/Q
kZhjewMrisVyDST7H6JiQeLRCQKBgQC2iolofzxKamC9qqft//MaHYIGyhpZIBjw
uDY9xiSaupyq8PCJupm5IjAPjzddllkg/u7AgxNcXCT/j+m5CurVGfU38CmeiA/G
FumePJltbMChwmJOBhR1/Pf38H0sOT+GWWm8Zp56qLu2PHMhOiWLpcTChtyY6RCu
cmI+gDX5pwKBgQDZfLKmgZOHBZloxeTdjpUGOUrt79uzo1fYcJE2TrfItHJ/PHst
ANByWj5Pj//SHw9BRKqbCN7shO8dvXr0SSCWT+WT1yWAuQw97cjoVVSC+BoFaEIS
lcihcJf46l4Qz6yXnKwRgU7+EVADO5AWzK0TG2UaQwnhTnW2AVyWGd6/uQKBgCD/
ZB1+uqBdIIXIvriYRxLWeLXPfPnVgoypUCzUj7NkvoNsMJ9FGxrLYlK9ue1yC0Sa
sQAd8bcFdS65ji+EGLVTDvAAuxVMVUWiyLJxE+8i9KOJJwohsTsu8ohgcdA3vRMF
QMbs6gMUU3J69PjxsO3QloPvqxr9LqXKSZYwyRELAoGAMas7IzrV6dpzODB2p2Dv
Zh/K0pptx//kYOXBQ78yCDh3dhEWZY+pjMyN0DGbPDoURnm7tGLC9WMMr/VeOKt1
IMON5ez45MMXM3TaEQwb4p38OywG+H1Q3cBrEqlnVWW8gKGDgqn8KABu4hMYq1HK
oVPRmf8RaEUEdssQ7Xm/m3E=
-----END PRIVATE KEY-----";

const TEST_RSA_PUBLIC_KEY: &str = "m6KbCqXkrPIoBUNfNshI90G6HijNVKeFytm3ATzfzMfFgqDUx9lHb86BZ1cvFFJ6CEUl_0-yUHdnZO8pa1ENUIT4FCYHkPTvfRkkl03ZxmWzSCpmIqLerBoyxgE-z2DwNPnH3KOwtnipxREGDeTRsoP4SZ8IUcbFXi10a58l3AI-toMqFEnuMTmcicEXA0k9-3RjFId-n5OJvqn1ucNIk04ZvvH4fkl1_rF98xou00MRniix_p6S6Os-bn875NHpdoEEQ0IRClg9GZp_prgRFmG8xYaYqJM7VlVz7AdSWDsP62G4JcuK1TUzARWPTv-5wW_Pb2CKDclS057WXyUd3w";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    email: String,
    name: String,
    exp: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Key {
    kid: String,
    alg: String,
    n: String,
    e: String,
    kty: String,
    r#use: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Certs {
    keys: Vec<Key>,
}

impl Certs {
    pub fn get(self, kid: &str) -> Result<DecodingKey, Box<dyn Error>> {
        for key in self.keys {
            if key.kid == *kid {
                return Ok(DecodingKey::from_rsa_components(&key.n, &key.e)?);
            }
        }
        Err("missing".into())
    }
}

fn main() {
    let encoding_key = EncodingKey::from_rsa_pem(TEST_RSA_PRIVATE_KEY.as_bytes()).unwrap();
    let mut header = Header::new(jsonwebtoken::Algorithm::RS256);
    header.kid = Some("1".into());
    let claims = Claims {
        sub: "1".into(),
        email: "user@email.com".into(),
        name: "User Bob".into(),
        exp: 2524636800,
    };
    let jwt = encode(&header, &claims, &encoding_key).unwrap();
    println!("{:?}", jwt);

    let certs = Certs {
        keys: vec![Key {
            alg: "RS256".into(),
            kid: "1".into(),
            n: TEST_RSA_PUBLIC_KEY.into(),
            e: "AQAB".into(),
            kty: "RSA".into(),
            r#use: "sig".into(),
        }],
    };
    let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
    validation.set_audience(&[
        "981002175662-g8jr2n89bptsn8n9ds1fn5edfheojr7i.apps.googleusercontent.com",
    ]);
    let decoding_key = certs.get("1").unwrap();
    let output_claims = decode::<Claims>(&jwt, &decoding_key, &validation).unwrap();
    println!("{:?}", output_claims)
}
