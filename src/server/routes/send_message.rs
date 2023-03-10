use actix_web::{Scope, web, HttpResponse};
use serde_json::json;
use crate::{notification_hub::NotificationHub, server::entities::{SendMessagePayload, SendMessageResponse}, entities::SenderResponse};

pub fn make_send_message_scope() -> Scope {
    web::scope("/messages")
        .service(web::resource("/send")
            .route(web::post().to(send_message)))
}

pub async fn send_message(hub_: web::Data<NotificationHub>, payload: web::Json<SendMessagePayload>) -> HttpResponse {
    let message = payload.into_inner();
    let hub = hub_.into_inner();

    let sender_responses = match hub.send_message(&message.destinatairies, &message.message, message.header).await {
        Ok(s) => s,
        Err(error) => {
            let body = json!({
                "message": format!("An error ocurred while sending messages: {error:?}")
            });

            let response = HttpResponse::InternalServerError().json(body);
            return response;
        }
    };
    
    let errors: Vec<_> = sender_responses.iter().filter_map(|sender_response|{
        match sender_response {
            SenderResponse::Err{..} => Some(sender_response),
            _ => None
        }
    }).collect();

    let responses:Vec<_> = message.destinatairies.iter().zip(&sender_responses).map(|(destinataire, response)|{
        SendMessageResponse::new(destinataire, response)
    }).collect();

    let response =
        if errors.len() > 0 {
            HttpResponse::InternalServerError().json(responses)
        } else {
            HttpResponse::Ok().json(responses)
        };

    response
}