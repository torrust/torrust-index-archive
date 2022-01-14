use actix_web::{web, Responder, HttpResponse, HttpRequest};
use serde::{Deserialize, Serialize};
use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Pbkdf2,
};
use std::borrow::Cow;
use crate::errors::{ServiceResult, ServiceError};
use crate::common::WebAppData;
use jsonwebtoken::{DecodingKey, decode, Validation, Algorithm};
use crate::models::response::OkResponse;
use crate::models::response::TokenResponse;
use crate::mailer::VerifyClaims;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(web::resource("/register").route(web::post().to(register)))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/verify/{token}").route(web::get().to(verify_user)))
    );
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Register {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Login {
    pub login: String,
    pub password: String,
}

pub async fn register(req: HttpRequest, payload: web::Json<Register>, app_data: WebAppData) -> ServiceResult<impl Responder> {
    if payload.password != payload.confirm_password {
        return Err(ServiceError::PasswordsDontMatch);
    }

    let password_length = payload.password.len();
    if password_length <= app_data.cfg.auth.min_password_length {
        return Err(ServiceError::PasswordTooShort);
    }
    if password_length >= app_data.cfg.auth.max_password_length {
        return Err(ServiceError::PasswordTooLong);
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash;
    if let Ok(password) = Pbkdf2.hash_password(payload.password.as_bytes(), &salt) {
        password_hash = password.to_string();
    } else {
        return Err(ServiceError::InternalServerError);
    }

    if payload.username.contains('@') {
        return Err(ServiceError::UsernameInvalid)
    }

    let res = sqlx::query!(
        "INSERT INTO torrust_users (username, email, password) VALUES ($1, $2, $3)",
        payload.username,
        payload.email,
        password_hash,
    )
        .execute(&app_data.database.pool)
        .await;

    if let Err(sqlx::Error::Database(err)) = res {
        return if err.code() == Some(Cow::from("2067")) {
            if err.message().contains("torrust_users.username") {
                Err(ServiceError::UsernameTaken)
            } else if err.message().contains("torrust_users.email") {
                Err(ServiceError::EmailTaken)
            } else {
                Err(ServiceError::InternalServerError)
            }
        } else {
            Err(sqlx::Error::Database(err).into())
        };
    }

    let conn_info = req.connection_info();

    if app_data.cfg.mail.email_verification_enabled {
        // todo: congig to enable/disable email verification
        let mail_res = app_data.mailer.send_verification_mail(
            &payload.email,
            &payload.username,
            format!("{}://{}", conn_info.scheme(), conn_info.host()).as_str()
        )
            .await;

        // get user id from user insert res
        let user_id = res.unwrap().last_insert_rowid();

        if mail_res.is_err() {
            app_data.database.delete_user(user_id).await;
            return Err(ServiceError::FailedToSendVerificationEmail)
        }
    }

    Ok(HttpResponse::Ok())
}

pub async fn login(payload: web::Json<Login>, app_data: WebAppData) -> ServiceResult<impl Responder> {
    let res = if payload.login.contains('@') {
        app_data.database.get_user_with_email(&payload.login).await
    } else {
        app_data.database.get_user_with_username(&payload.login).await
    };

    match res {
        Some(user) => {
            if !user.email_verified {
                return Err(ServiceError::EmailNotVerified)
            }

            let parsed_hash = PasswordHash::new(&user.password)?;

            if !Pbkdf2.verify_password(payload.password.as_bytes(), &parsed_hash).is_ok() {
                return Err(ServiceError::WrongPasswordOrUsername);
            }

            let username = user.username.clone();
            let token = app_data.auth.sign_jwt(user.clone());


            Ok(HttpResponse::Ok().json(OkResponse {
                data: TokenResponse {
                    token,
                    username,
                    admin: user.administrator
                }
            }))
        }
        None => Err(ServiceError::WrongPasswordOrUsername)
    }
}

pub async fn verify_user(req: HttpRequest, app_data: WebAppData) -> String {
    let token = req.match_info().get("token").unwrap();

    let token_data = match decode::<VerifyClaims>(
        token,
        &DecodingKey::from_secret(app_data.cfg.auth.secret_key.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => {
            if !token_data.claims.iss.eq("email-verification") {
                return ServiceError::TokenInvalid.to_string()
            }

            token_data.claims
        },
        Err(_) => return ServiceError::TokenInvalid.to_string()
    };

    let res = sqlx::query!(
        "UPDATE torrust_users SET email_verified = TRUE WHERE username = ?",
        token_data.sub
    )
        .execute(&app_data.database.pool)
        .await;

    if let Err(_) = res {
        return ServiceError::InternalServerError.to_string()
    }

    String::from("Email verified, you can close this page.")
}

pub async fn me(req: HttpRequest, app_data: WebAppData) -> ServiceResult<impl Responder> {
    let user = match app_data.auth.get_user_from_request(&req).await {
        Ok(user) => Ok(user),
        Err(e) => Err(e)
    }?;

    let username = user.username.clone();
    let token = app_data.auth.sign_jwt(user.clone());

    Ok(HttpResponse::Ok().json(OkResponse {
        data: TokenResponse {
            token,
            username,
            admin: user.administrator
        }
    }))
}
