mod routes;
mod entities;
use actix_web::{web, HttpServer, App};
use crate::{notification_hub::NotificationHub, server::routes::{health_check, make_send_message_scope}};



pub async fn start_server(hub: NotificationHub, host: &str, port: u16) -> anyhow::Result<()> {
    log::info!("Starting server on {host}:{port}");
    let hub_data = web::Data::new(hub);

    HttpServer::new(move || {
        App::new()
            .app_data(hub_data.clone())
            .service(health_check)
            .service(make_send_message_scope())
    })
    .bind((host, port))?
    .run()
    .await?;

    log::info!("Server finished listen on {host}:{port}");
    Ok(())
}