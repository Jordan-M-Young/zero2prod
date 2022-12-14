use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    format!("Welcom {}!", form.email);
    format!("Wecom {}!", form.name);
    HttpResponse::Ok().finish()
}
