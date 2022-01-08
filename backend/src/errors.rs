use derive_more::{Display, Error};
use actix_web::{ResponseError, HttpResponse, HttpResponseBuilder};
use actix_web::http::{header, StatusCode};
use serde::{Deserialize, Serialize};
use std::error;

pub type ServiceResult<V> = std::result::Result<V, ServiceError>;

#[derive(Debug, Display, PartialEq, Error)]
#[allow(dead_code)]
pub enum ServiceError {
    #[display(fmt = "internal server error")]
    InternalServerError,

    #[display(
    fmt = "This server is is closed for registration. Contact admin if this is unexpecter"
    )]
    ClosedForRegistration,

    #[display(fmt = "The value you entered for email is not an email")] //405j
    NotAnEmail,
    #[display(fmt = "The value you entered for URL is not a URL")] //405j
    NotAUrl,

    #[display(fmt = "Invalid username/email or password")]
    WrongPasswordOrUsername,
    #[display(fmt = "Username not found")]
    UsernameNotFound,
    #[display(fmt = "Account not found")]
    AccountNotFound,

    /// when the value passed contains profainity
    #[display(fmt = "Can't allow profanity in usernames")]
    ProfainityError,
    /// when the value passed contains blacklisted words
    /// see [blacklist](https://github.com/shuttlecraft/The-Big-Username-Blacklist)
    #[display(fmt = "Username contains blacklisted words")]
    BlacklistError,
    /// when the value passed contains characters not present
    /// in [UsernameCaseMapped](https://tools.ietf.org/html/rfc8265#page-7)
    /// profile
    #[display(fmt = "username_case_mapped violation")]
    UsernameCaseMappedError,

    #[display(fmt = "Passsword too short")]
    PasswordTooShort,
    #[display(fmt = "Username too long")]
    PasswordTooLong,
    #[display(fmt = "Passwords don't match")]
    PasswordsDontMatch,

    /// when the a username is already taken
    #[display(fmt = "Username not available")]
    UsernameTaken,

    #[display(fmt = "Username contains illegal characters")]
    UsernameInvalid,

    /// email is already taken
    #[display(fmt = "Email not available")]
    EmailTaken,

    #[display(fmt = "Please verify your email before logging in")]
    EmailNotVerified,

    /// when the a token name is already taken
    /// token not found
    #[display(fmt = "Token not found. Please sign in.")]
    TokenNotFound,

    /// token expired
    #[display(fmt = "Token expired. Please sign in again.")]
    TokenExpired,

    #[display(fmt = "Token invalid.")]
    /// token invalid
    TokenInvalid,

    #[display(fmt = "Torrent not found.")]
    TorrentNotFound,

    #[display(fmt = "Uploaded torrent is not valid")]
    InvalidTorrentFile,

    #[display(fmt = "Only .torrent files can be uploaded")]
    InvalidFileType,

    #[display(fmt = "Bad request.")]
    BadRequest,

    #[display(fmt = "Selected category does not exist")]
    InvalidCategory,

    #[display(fmt = "Unauthorized action.")]
    Unauthorized,

    #[display(fmt = "This torrent already exists in our database.")]
    InfoHashAlreadyExists,

    #[display(fmt = "Sorry, we have an error with our tracker connection.")]
    TrackerOffline,

    #[display(fmt = "Failed to send the verification email.")]
    FailedToSendVerificationEmail,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorToResponse {
    pub error: String,
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::ClosedForRegistration => StatusCode::FORBIDDEN,
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::NotAnEmail => StatusCode::BAD_REQUEST,
            ServiceError::NotAUrl => StatusCode::BAD_REQUEST,
            ServiceError::WrongPasswordOrUsername => StatusCode::FORBIDDEN,
            ServiceError::UsernameNotFound => StatusCode::NOT_FOUND,
            ServiceError::AccountNotFound => StatusCode::NOT_FOUND,

            ServiceError::ProfainityError => StatusCode::BAD_REQUEST,
            ServiceError::BlacklistError => StatusCode::BAD_REQUEST,
            ServiceError::UsernameCaseMappedError => StatusCode::BAD_REQUEST,

            ServiceError::PasswordTooShort => StatusCode::BAD_REQUEST,
            ServiceError::PasswordTooLong => StatusCode::BAD_REQUEST,
            ServiceError::PasswordsDontMatch => StatusCode::BAD_REQUEST,

            ServiceError::UsernameTaken => StatusCode::BAD_REQUEST,
            ServiceError::UsernameInvalid => StatusCode::BAD_REQUEST,
            ServiceError::EmailTaken => StatusCode::BAD_REQUEST,
            ServiceError::EmailNotVerified => StatusCode::FORBIDDEN,

            ServiceError::TokenNotFound => StatusCode::UNAUTHORIZED,
            ServiceError::TokenExpired => StatusCode::UNAUTHORIZED,
            ServiceError::TokenInvalid => StatusCode::UNAUTHORIZED,

            ServiceError::TorrentNotFound => StatusCode::BAD_REQUEST,

            ServiceError::InvalidTorrentFile => StatusCode::BAD_REQUEST,
            ServiceError::InvalidFileType => StatusCode::BAD_REQUEST,

            ServiceError::BadRequest => StatusCode::BAD_REQUEST,

            ServiceError::InvalidCategory => StatusCode::BAD_REQUEST,

            ServiceError::Unauthorized => StatusCode::UNAUTHORIZED,

            ServiceError::InfoHashAlreadyExists => StatusCode::BAD_REQUEST,

            ServiceError::TrackerOffline => StatusCode::INTERNAL_SERVER_ERROR,

            ServiceError::FailedToSendVerificationEmail => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .append_header((header::CONTENT_TYPE, "application/json; charset=UTF-8"))
            .body(
                serde_json::to_string(&ErrorToResponse {
                    error: self.to_string(),
                })
                    .unwrap(),
            )
            .into()
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(e: sqlx::Error) -> Self {
        eprintln!("{}", e);
        ServiceError::InternalServerError
    }
}

impl From<pbkdf2::password_hash::Error> for ServiceError {
    fn from(e: pbkdf2::password_hash::Error) -> Self {
        eprintln!("{}", e);
        ServiceError::InternalServerError
    }
}

impl From<std::io::Error> for ServiceError {
    fn from(e: std::io::Error) -> Self {
        eprintln!("{}", e);
        ServiceError::InternalServerError
    }
}

impl From<Box<dyn error::Error>> for ServiceError {
    fn from(e: Box<dyn error::Error>) -> Self {
        eprintln!("{}", e);
        ServiceError::InternalServerError
    }
}
