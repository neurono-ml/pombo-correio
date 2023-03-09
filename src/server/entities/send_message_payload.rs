use serde::Deserialize;

use crate::entities::DestinataryType;

#[derive(Deserialize)]
pub struct SendMessagePayload {
    pub destinatairies: Vec<DestinataryType>,
    pub header: Option<String>,
    pub message: String,
}