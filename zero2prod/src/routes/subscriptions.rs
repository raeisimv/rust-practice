use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct SubscriptionFormData {
    name: String,
    email: String,
}

pub async fn subscriptions(body: web::Form<SubscriptionFormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
