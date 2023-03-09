use clap::{Parser};
use command_line::CommandLine;
use notification_hub::NotificationHub;
use server::start_server;

mod entities;
mod server;
mod command_line;
mod notification_hub;


#[tokio::main(flavor="current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let arguments = CommandLine::parse();
    
    let hub = NotificationHub::load(&arguments.hub_configuration_file)?;

    start_server(hub, &arguments.host, arguments.port).await?;

    Ok(())
}
