use actix_web::{HttpResponse, get, web};
use serde_json::json;

use crate::notification_hub::NotificationHub;


#[get("/health")]
pub(super) async fn health_check(hub_: web::Data<NotificationHub>) -> actix_web::Result<HttpResponse> {
    let hub = hub_.into_inner();
    
    let response = match hub.health_check().await {
        Ok(versions) => HttpResponse::Ok().json(versions),
        Err(error) => {
            let error_message = format!("An error occurred while calling kuberentes API: {error:?}");
            let response_body = json!({
                "message": error_message
            });
            HttpResponse::InternalServerError().json(response_body)
        }
    };

    Ok(response)
}