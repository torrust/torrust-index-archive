use actix_web::HttpRequest;
use crate::models::user::{Claims, User};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, encode, Header, EncodingKey};
use crate::utils::time::current_time;
use crate::errors::ServiceError;
use std::sync::Arc;
use crate::database::Database;
use crate::config::TorrustConfig;

pub struct AuthorizationService {
    cfg: Arc<TorrustConfig>,
    database: Arc<Database>,
}

impl AuthorizationService {
    pub fn new(cfg: Arc<TorrustConfig>, database: Arc<Database>) -> AuthorizationService {
        AuthorizationService {
            cfg,
            database
        }
    }

    pub fn sign_jwt(&self, user: User) -> String {
        // create JWT that expires in two weeks
        let key = self.cfg.auth.secret_key.as_bytes();
        let exp_date = current_time() + 1_209_600; // two weeks from now

        let claims = Claims {
            sub: user.username,
            admin: user.administrator,
            exp: exp_date,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(key),
        )
            .unwrap();

        token
    }

    pub fn verify_jwt(&self, token: &str) -> Result<Claims, ServiceError> {
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.cfg.auth.secret_key.as_bytes()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token_data) => {
                if token_data.claims.exp < current_time() {
                    return Err(ServiceError::TokenExpired)
                }
                Ok(token_data.claims)
            },
            Err(_) => Err(ServiceError::TokenInvalid)
        }
    }

    pub fn get_claims_from_request(&self, req: &HttpRequest) -> Result<Claims, ServiceError> {
        let _auth = req.headers().get("Authorization");
        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();

                match self.verify_jwt(token) {
                    Ok(claims) => Ok(claims),
                    Err(e) => Err(e),
                }
            }
            None => Err(ServiceError::TokenNotFound)
        }
    }

    pub async fn get_user_from_request(&self, req: &HttpRequest) -> Result<User, ServiceError> {
        let claims = match self.get_claims_from_request(req) {
            Ok(claims) => Ok(claims),
            Err(e) => Err(e)
        }?;

        match self.database.get_user_with_username(&claims.sub).await {
            Some(user) => Ok(user),
            None => Err(ServiceError::AccountNotFound)
        }
    }
}
