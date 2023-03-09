mod routes;
mod entities;
use crate::notification_hub::NotificationHub;


pub async fn start_server(hub: NotificationHub, host: &str, port: u16) -> anyhow::Result<()> {
    todo!()
}