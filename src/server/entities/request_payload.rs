use super::email_message::EmailMessage;
use serde::Deserialize;


#[derive(Deserialize)]
pub enum RequestPayload {
    Email(EmailMessage)
}