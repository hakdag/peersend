use std::io::Error;

use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,     // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,     // Optional. Issued at (as UTC timestamp)
    iss: String,    // Optional. Issuer
    sub: String,    // Optional. Subject (whom token refers to)
}

pub struct TokenHandler {}

impl TokenHandler {
    pub fn generate(sub: String) -> Result<String, Error> {
        let key = b"fCGikre1TAc4apI1k8YvcyWorpXs8mLa";
        let my_claims = Claims {
            sub: sub.to_owned(),
            exp: 1734086050,
            iat: 1733999650,
            iss: "peersend".to_owned(),
        };
        match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)) {
            Ok(t) => Ok(t),
            Err(_) => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Token generation has failed.").to_string()))
            /*
            Err(e) => match *e.kind() {
                ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
                _ => panic!("Some other errors"),
            },
            */
        }     
    }
}
