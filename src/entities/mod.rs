mod sender_response;
mod message_destinatary;

pub use message_destinatary::DestinataryType;
pub use sender_response::SenderResponse;

pub use message_destinatary::{EmailDestinatary, SlackDestinatary};