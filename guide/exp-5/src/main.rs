use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[paralegal::marker(sensitive)]
struct Claims {
    aud: String,
    sub: String,
    company: String,
    exp: u64,
}

#[paralegal::marker(sink, arguments = [0])]
fn send(_token: &Claims) {
    todo!()
}

#[paralegal::marker(process, arguments = [0])]
fn process_claims(mut _claims: Claims) -> Claims {
    _claims.exp = 0;
    _claims
}

#[paralegal::analyze]
fn analyze_func() {
    let key = b"secret";
    let my_claims = Claims {
        aud: "me".to_owned(),
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        exp: 10000000000,
    };
    let after_claims = process_claims(my_claims);
    send(&after_claims);
}

fn main() {
    todo!()
}
