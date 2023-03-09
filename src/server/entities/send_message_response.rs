use serde::{Serialize};

use crate::entities::{DestinataryType, SenderResponse};

#[derive(Serialize)]
pub struct SendMessageResponse {
    destinatare: DestinataryType,
    sender_response: SenderResponse,
}

impl SendMessageResponse {
    pub fn new(destinataire: &DestinataryType, sender_response: &SenderResponse) -> Self {
        SendMessageResponse {
            destinatare: destinataire.to_owned(),
            sender_response: sender_response.to_owned()
        }
    }
}