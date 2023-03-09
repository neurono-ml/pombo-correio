use clap::{Parser};
use command_line::CommandLine;
use notification_hub::NotificationHub;
use server::start_server;

mod server;
mod command_line;
mod notification_hub;


#[tokio::main(flavor="current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let command_line = CommandLine::parse();
    let hub = NotificationHub::new();

    start_server(hub, &command_line.host, command_line.port).await?;

    Ok(())
}
