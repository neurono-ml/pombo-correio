use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Deserialize, Serialize)]
pub struct EmailDestinatary {
    pub configuration: String,
}
