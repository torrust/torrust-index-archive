use crate::config::TorrustConfig;
use std::sync::Arc;
use crate::errors::ServiceError;
use serde::{Serialize, Deserialize};
use lettre::{AsyncSmtpTransport, Tokio1Executor, Message, AsyncTransport};
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::message::{MessageBuilder, MultiPart, SinglePart};
use jsonwebtoken::{encode, Header, EncodingKey};
use sailfish::TemplateOnce;
use crate::utils::time::current_time;

pub struct MailerService {
    cfg: Arc<TorrustConfig>,
    mailer: Arc<Mailer>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyClaims {
    pub iss: String,
    pub sub: String,
    pub exp: u64,
}

#[derive(TemplateOnce)]
#[template(path = "../templates/verify.html")]
struct VerifyTemplate {
    username: String,
    verification_url: String,
}


impl MailerService {
    pub fn new(cfg: Arc<TorrustConfig>) -> MailerService {
        let mailer = Arc::new(Self::get_mailer(&cfg));

        Self {
            cfg,
            mailer,
        }
    }

    fn get_mailer(cfg: &TorrustConfig) -> Mailer {
        let creds = Credentials::new(cfg.mail.username.to_owned(), cfg.mail.password.to_owned());

        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&cfg.mail.server)
            .port(cfg.mail.port)
            .credentials(creds)
            .authentication(vec![
                Mechanism::Login,
                Mechanism::Xoauth2,
                Mechanism::Plain,
            ])
            .build()
    }

    pub async fn send_verification_mail(&self, to: &str, username: &str, base_url: &str) -> Result<(), ServiceError> {
        let builder = self.get_builder(to);
        let verification_url = self.get_verification_url(username, base_url);

        let mail_body = format!(
            r#"
Welcome to Torrust, {}!

Please click the confirmation link below to verify your account.
{}

If this account wasn't made by you, you can ignore this email.
            "#,
            username,
            verification_url
        );

        let ctx = VerifyTemplate {
            username: String::from(username),
            verification_url,
        };

        let mail = builder
            .subject("Torrust - Email verification")
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(lettre::message::header::ContentType::TEXT_PLAIN)
                            .body(mail_body)
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(lettre::message::header::ContentType::TEXT_HTML)
                            .body(ctx.render_once().unwrap())
                    )
            )
            .unwrap();

        match self.mailer.send(mail).await {
            Ok(_res) => Ok(()),
            Err(e) => {
                eprintln!("Failed to send email: {}", e);
                Err(ServiceError::FailedToSendVerificationEmail)
            },
        }
    }

    fn get_builder(&self, to: &str) -> MessageBuilder {
        Message::builder()
            .from(self.cfg.mail.from.parse().unwrap())
            .reply_to(self.cfg.mail.reply_to.parse().unwrap())
            .to(to.parse().unwrap())
    }

    fn get_verification_url(&self, username: &str, base_url: &str) -> String {
        // create verification JWT
        let key = self.cfg.auth.secret_key.as_bytes();

        // Create non expiring token that is only valid for email-verification
        let claims = VerifyClaims {
            iss: String::from("email-verification"),
            sub: String::from(username),
            exp: current_time() + 315_569_260 // 10 years from now
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(key),
        )
            .unwrap();

        let mut base_url = base_url.clone();
        if let Some(cfg_base_url) = &self.cfg.net.base_url {
            base_url = cfg_base_url;
        }

        format!("{}/user/verify/{}", base_url, token)
    }
}

pub type Mailer = AsyncSmtpTransport<Tokio1Executor>;
