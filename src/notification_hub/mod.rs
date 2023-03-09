mod messages;
mod email_client;
mod slack_client;
use anyhow::Ok;
use serde::Deserialize;

use crate::entities::{DestinataryType, SenderResponse};

use self::{slack_client::SlackClient, email_client::EmailClient, messages::{SLACK_CLIENT_UNAVAILABLE, EMAIL_CLIENT_UNAVAILABLE}};

#[derive(Deserialize)]
pub struct NotificationHub {
    slack: Option<SlackClient>,
    email: Option<EmailClient>,
}

impl NotificationHub {
    pub fn load(source_path: &str) -> anyhow::Result<NotificationHub> {
        log::info!("Loading hub configuration file from {source_path}");

        let content = std::fs::read_to_string(source_path)?;
        let hub: NotificationHub = serde_yaml::from_str(&content)?;

        let capabilities = hub.get_capabilities().join(", ");
        log::info!("Hub loaded with the following capabilities: {capabilities}");

        Ok(hub)
    }
    
    pub fn get_capabilities(&self) -> Vec<String> {
        let mut capabilities = Vec::new();

        if self.slack.is_some() {
            capabilities.push("slack".to_owned());
        }

        if self.email.is_some() {
            capabilities.push("email".to_owned());
        }

        capabilities
    }

    ///TODO: implement health check for slack API and mail server somehow
    pub async fn health_check(&self) -> anyhow::Result<()>{
        Ok(())
    }

    pub async fn send_message(&self, destinatairies: &Vec<DestinataryType>, message: &str, 
                                header: Option<String>) -> anyhow::Result<Vec<SenderResponse>> {

        let mut responses = Vec::new();

        for destinatary_type in destinatairies.iter() {         
            
            let response = match destinatary_type {
                DestinataryType::Slack(destinatary) => {
                    if self.slack.is_none() {
                        log::warn!("Unavailable client called: {SLACK_CLIENT_UNAVAILABLE}");
                        SenderResponse::UnavailableClientType(SLACK_CLIENT_UNAVAILABLE.to_owned())
                    } else {
                        if let Some(header_value) = header.clone() {
                            let new_body = header_value + "\n" + message;
                            self.slack.clone().unwrap().send_message(destinatary, &new_body).await?
                        } else {
                            self.slack.clone().unwrap().send_message(destinatary, message).await?
                        }
                    }
                },
                DestinataryType::Email(destinatary) =>
                    if self.email.is_none() {
                        log::warn!("Unavailable client called: {EMAIL_CLIENT_UNAVAILABLE}");
                        SenderResponse::UnavailableClientType(EMAIL_CLIENT_UNAVAILABLE.to_owned())
                    } else {
                        self.email.clone().unwrap().send_message(destinatary, message, header.clone()).await?
                    }
            };

            responses.push(response);
        };

        Ok(responses)
    }
}