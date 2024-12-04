use actix_web::{HttpResponse, Responder};

/// Handles health check requests.
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
