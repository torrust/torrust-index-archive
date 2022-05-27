use crate::config::Configuration;
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
    cfg: Arc<Configuration>,
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
    pub async fn new(cfg: Arc<Configuration>) -> MailerService {
        let mailer = Arc::new(Self::get_mailer(&cfg).await);

        Self {
            cfg,
            mailer,
        }
    }

    async fn get_mailer(cfg: &Configuration) -> Mailer {
        let settings = cfg.settings.read().await;

        let creds = Credentials::new(settings.mail.username.to_owned(), settings.mail.password.to_owned());

        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&settings.mail.server)
            .port(settings.mail.port)
            .credentials(creds)
            .authentication(vec![
                Mechanism::Login,
                Mechanism::Xoauth2,
                Mechanism::Plain,
            ])
            .build()
    }

    pub async fn send_verification_mail(&self, to: &str, username: &str, base_url: &str) -> Result<(), ServiceError> {
        let builder = self.get_builder(to).await;
        let verification_url = self.get_verification_url(username, base_url).await;

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

    async fn get_builder(&self, to: &str) -> MessageBuilder {
        let settings = self.cfg.settings.read().await;

        Message::builder()
            .from(settings.mail.from.parse().unwrap())
            .reply_to(settings.mail.reply_to.parse().unwrap())
            .to(to.parse().unwrap())
    }

    async fn get_verification_url(&self, username: &str, base_url: &str) -> String {
        let settings = self.cfg.settings.read().await;

        // create verification JWT
        let key = settings.auth.secret_key.as_bytes();

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

        //let mut base_url = base_url.clone();
        let base_url = &settings.net.base_url;

        format!("{}/user/verify/{}", base_url, token)
    }
}

pub type Mailer = AsyncSmtpTransport<Tokio1Executor>;
