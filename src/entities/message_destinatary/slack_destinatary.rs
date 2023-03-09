use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Deserialize, Serialize)]
pub struct SlackDestinatary {
    pub webhook: String,
}
