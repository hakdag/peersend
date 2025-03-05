use std::io::Error;
use std::convert::TryFrom;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,     // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,     // Issued at (as UTC timestamp)
    iss: String,    // Issuer
    sub: String,    // Subject (whom token refers to)
    mac: Option<String>, // MAC address of the device
}

pub struct TokenHandler {
    key: [u8; 32]
}

impl TokenHandler {
    pub fn new() -> Self {
        Self { key: *b"fCGikre1TAc4apI1k8YvcyWorpXs8mLa" }
    }

    pub fn generate(&self, email: &String, mac: Option<String>) -> Result<String, Error> {
        let date_time = Utc::now();
        let tomorrow = date_time + chrono::Duration::days(1);
        let iat = usize::try_from(date_time.timestamp()).unwrap();
        let exp = usize::try_from(tomorrow.timestamp()).unwrap();
        let my_claims = Claims {
            sub: email.to_owned(),
            exp: exp,
            iat: iat,
            iss: "peersend".to_owned(),
            mac: mac
        };
        match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(&self.key)) {
            Ok(t) => Ok(t),
            Err(_) => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Token generation has failed.").to_string()))
        }     
    }

    pub fn validate(&self, token: String) -> Result<String, Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_required_spec_claims(&["exp", "sub", "iat", "iss"]);
        match decode::<Claims>(&token, &DecodingKey::from_secret(&self.key), &validation) {
            Ok(claims) => Ok(claims.claims.sub),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Token is invalid.").to_string())),
                ErrorKind::InvalidIssuer => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Issuer is invalid.").to_string())),
                _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Unknown token error.").to_string())),
            },
        }
    }
}
