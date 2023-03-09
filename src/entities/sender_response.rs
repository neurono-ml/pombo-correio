use serde::Serialize;

#[derive(Clone)]
#[derive(Serialize)]
pub enum SenderResponse {
    Succeed,
    DestinataryNotFound(String),
    UnavailableClientType(String),
    Err{message: String}
}