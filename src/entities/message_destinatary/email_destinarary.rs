use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Deserialize, Serialize)]
pub struct EmailDestinatary {
    recipient: String,
}
