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
fn send(_token: &str) {
    todo!()
}

fn fake_encode(_claims: &Claims) -> String {
    println!("{:?}", _claims);
    return _claims.aud.clone()
}

#[paralegal::analyze]
fn analyze_func1(key: &[u8], my_claims: &Claims) {
    let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };
    send(&token);
}

// Comment this back in to make the policy fail
// #[paralegal::analyze]
// fn analyze_func2(key: &[u8], my_claims: &Claims) {
//     let token = fake_encode(&my_claims);
//     send(&token);
// }

fn main() {
    let key = b"secret";
    let my_claims = Claims {
        aud: "me".to_owned(),
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        exp: 10000000000,
    };

    analyze_func1(key, &my_claims);
}
