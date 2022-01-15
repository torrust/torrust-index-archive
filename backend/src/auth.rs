use actix_web::HttpRequest;
use crate::models::user::{Claims, User};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, encode, Header, EncodingKey};
use crate::utils::time::current_time;
use crate::errors::ServiceError;
use std::sync::Arc;
use crate::database::Database;
use crate::config::Configuration;

pub struct AuthorizationService {
    cfg: Arc<Configuration>,
    database: Arc<Database>,
}

impl AuthorizationService {
    pub fn new(cfg: Arc<Configuration>, database: Arc<Database>) -> AuthorizationService {
        AuthorizationService {
            cfg,
            database
        }
    }

    pub async fn sign_jwt(&self, user: User) -> String {
        let settings = self.cfg.settings.read().await;

        // create JWT that expires in two weeks
        let key = settings.auth.secret_key.as_bytes();
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

    pub async fn verify_jwt(&self, token: &str) -> Result<Claims, ServiceError> {
        let settings = self.cfg.settings.read().await;

        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(settings.auth.secret_key.as_bytes()),
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

    pub async fn get_claims_from_request(&self, req: &HttpRequest) -> Result<Claims, ServiceError> {
        let _auth = req.headers().get("Authorization");
        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();

                match self.verify_jwt(token).await {
                    Ok(claims) => Ok(claims),
                    Err(e) => Err(e),
                }
            }
            None => Err(ServiceError::TokenNotFound)
        }
    }

    pub async fn get_user_from_request(&self, req: &HttpRequest) -> Result<User, ServiceError> {
        let claims = match self.get_claims_from_request(req).await {
            Ok(claims) => Ok(claims),
            Err(e) => Err(e)
        }?;

        match self.database.get_user_with_username(&claims.sub).await {
            Some(user) => Ok(user),
            None => Err(ServiceError::AccountNotFound)
        }
    }
}
