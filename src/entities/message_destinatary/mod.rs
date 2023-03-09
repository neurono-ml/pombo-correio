use serde::{Deserialize, Serialize};

mod slack_destinatary;
mod email_destinarary;

pub use slack_destinatary::SlackDestinatary;
pub use email_destinarary::EmailDestinatary;

#[derive(Clone)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all="lowercase")]
#[serde(tag="type")]
pub enum DestinataryType {
    Slack(SlackDestinatary),
    Email(EmailDestinatary),
}