use actix_web::{HttpResponse, Responder, Result};


// TODO render index template
pub async fn index() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().body("hello"))
}
