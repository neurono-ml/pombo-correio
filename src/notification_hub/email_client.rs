use serde::Deserialize;
use crate::entities::{EmailDestinatary, SenderResponse};

#[derive(Deserialize, Clone)]
pub struct EmailClient {

}

impl EmailClient {
    pub async fn send_message(&self, _destinatary: &EmailDestinatary, _message: &str, _header: Option<String>) -> anyhow::Result<SenderResponse> {
        todo!()
    }
}