use std::collections::HashMap;

use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use crate::entities::{SlackDestinatary, SenderResponse};

const POMBO_CORREIO: &str = "pombo-correio";

#[derive(Deserialize, Clone)]
pub struct SlackClient {
    webhooks: HashMap<String, String>
}

impl SlackClient {
    pub async fn send_message(&self, destinatary: &SlackDestinatary, message: &str) -> anyhow::Result<SenderResponse> {
        let webhook = destinatary.webhook.clone();

        log::debug!("Preparing to send slack message to webhook {webhook}");
        let payload = json!({
            "text": message,
        });

        match self.webhooks.get(&webhook) {
            Some(url) => {
                let client = make_client()?;
                let response =  client
                    .post(url)
                    .json(&payload)
                    .send().await?;

                let status_succeed = response
                    .status()
                    .is_success();

                if status_succeed {
                    Ok(SenderResponse::Succeed)
                } else {
                    let message = response.text().await?;
                    Ok(SenderResponse::Err { message })
                }
            },
            None => Ok(SenderResponse::DestinataryNotFound(webhook)),
        }
    }    
}

pub fn make_client() ->  anyhow::Result<Client> {
    let client = Client::builder()
        .user_agent(POMBO_CORREIO)
        .build()?;

    Ok(client)
}