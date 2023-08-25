use std::env;
use chrono::{Duration, Utc};

use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserClaim {
    pub user_id: String,
    pub exp: usize
}

pub fn issue_access_token(user_id: String) -> String {
    let key = EncodingKey::from_secret(env::var("ACCESS_TOKEN_SECRET").unwrap().as_str().as_ref());

    println!("{}", (Utc::now() + Duration::minutes(15)).timestamp());

    let user_claim: UserClaim = UserClaim {
        user_id,
        exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
    };

    let token = encode(&Header::default(), &user_claim, &key);

    return token.unwrap();
}