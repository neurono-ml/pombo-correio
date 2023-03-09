use std::{collections::HashMap};
use gethostname::gethostname;
use lettre::{SmtpTransport, transport::{smtp::{authentication::Credentials, extension::ClientId}}, Message, message::header::{ContentType}, Transport};
use serde::Deserialize;
use crate::entities::{EmailDestinatary, SenderResponse};

const DEFAULT_PORT: u16 = 587;
const DEFAULT_SUBJECT: &str = "Message from Pombo";

#[derive(Deserialize, Clone)]
pub struct EmailClient {
    configurations: HashMap<String, EmailSender>
}

#[derive(Deserialize, Clone)]
pub struct EmailSender {
    from: EmailUser,
    to: EmailUser,
    transport: EmailTransport,
    default_subject: Option<String>
}

#[derive(Deserialize, Clone)]
struct EmailUser {
    name: Option<String>,
    email: String,
}

impl EmailUser {
    fn to_string(&self) -> String {
        match self.name.clone() {
            Some(name_) => format!("{name_} <{email}>", email=self.email),
            None => self.email.clone()
        }
    }
}

#[derive(Deserialize, Clone)]
struct EmailTransport {
    server_host: String,
    server_port: Option<u16>,

    username: String,
    password: String,
}

impl EmailTransport {
    fn make_client(&self) -> anyhow::Result<SmtpTransport> {
        let server_name = gethostname().to_str().unwrap_or("localhost").to_owned();
        let credentials = Credentials::new(self.username.clone(), self.password.clone());
        let hello_name = ClientId::Domain(server_name);

        let client = SmtpTransport::relay(&self.server_host)?
            .credentials(credentials)
            .port(self.server_port.unwrap_or(DEFAULT_PORT))
            .hello_name(hello_name)
            .build();
            
        Ok(client)
    }
}

impl EmailClient {
    pub async fn send_message(&self, destinatary: &EmailDestinatary, message: &str, header: Option<String>) -> anyhow::Result<SenderResponse> {
        let configuration_name = destinatary.configuration.clone();
        log::debug!("Preparing to send email message to {configuration_name}");

        match self.configurations.get(&configuration_name) {
            Some(sender) => {
                
                let to_mailbox = sender.to.to_string().parse()?;
                
                let configuration_subject = sender.default_subject.clone().unwrap_or(DEFAULT_SUBJECT.to_owned());
                let subject = header.unwrap_or(configuration_subject);

                let from_ = sender.from.clone();
                let from_mailbox = from_.to_string().parse()?;

                let message =
                    Message::builder()
                        .from(from_mailbox)
                        .to(to_mailbox)
                        .header(ContentType::TEXT_PLAIN)
                        .subject(subject)
                        .body(String::from(message))?;

                let transport = sender.transport.make_client()?;
                
                match transport.send(&message) {
                    Ok(_) => Ok(SenderResponse::Succeed),
                    Err(error) => {
                        let message = format!("An error occurred while sending the message: {error:?}");
                        Ok(SenderResponse::Err{message})
                    }
                }
            }
            None => Ok(SenderResponse::DestinataryNotFound(configuration_name))
        }
    }

}