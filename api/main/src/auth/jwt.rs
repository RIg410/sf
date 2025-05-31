use bson::oid::ObjectId;
use chrono::{Duration, Utc};
use eyre::{eyre, Error};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub struct Jwt {
    jwt_decode: DecodingKey,
    jwt_encode: EncodingKey,
    validation: Validation,
    header: Header,
}

impl Jwt {
    pub fn new(secret: &str) -> Self {
        let jwt_decode = DecodingKey::from_secret(secret.as_bytes());
        let jwt_encode = EncodingKey::from_secret(secret.as_bytes());
        let validation = Validation::default();
        let header = Header::default();
        Jwt {
            jwt_decode,
            jwt_encode,
            validation,
            header,
        }
    }

    pub fn claims<C: DeserializeOwned>(&self, header: &str) -> Result<(C, JwtToken), Error> {
        let jwt = header
            .strip_prefix("Bearer ")
            .ok_or_else(|| eyre!("No Bearer"))?;
        let token = jsonwebtoken::decode::<C>(jwt, &self.jwt_decode, &self.validation)?;
        Ok((
            token.claims,
            JwtToken {
                key: jwt.to_string(),
            },
        ))
    }

    pub fn make_jwt<C: Serialize>(&self, claims: C) -> Result<JwtToken, Error> {
        let key = jsonwebtoken::encode(&self.header, &claims, &self.jwt_encode)?;
        Ok(JwtToken { key })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct JwtToken {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: ObjectId,
    pub exp: u64,
}

impl Claims {
    pub fn new(id: ObjectId) -> Self {
        Claims {
            id,
            exp: (Utc::now() + Duration::days(360)).timestamp() as u64,
        }
    }
}
