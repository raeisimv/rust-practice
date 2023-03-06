use actix_web::{HttpResponse, web};
use sqlx::{PgPool};
use uuid::Uuid;
use chrono::Utc;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct SubscriptionFormData {
    name: String,
    email: String,
}

pub async fn subscriptions(body: web::Form<SubscriptionFormData>, conn: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_name = %body.name,
        subscriber_email = %body.email
    );
    let _request_span = request_span.enter();

    tracing::info!("request_id: {request_id} - Adding '{}', '{}' as a new subscriber", body.name, body.email);

    let at = Utc::now();
    let id = Uuid::new_v4();

    tracing::info!("request_id: {request_id} - Saving subscriber details into the database");

    let query_span = tracing::info_span!("Saving new subscriber details into the database");

    let res = sqlx::query!(r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
        id, body.email, body.name, at
    )
        .execute(conn.get_ref())
        .instrument(query_span)
        .await
        ;
    match res {
        Ok(_) => {
            tracing::info!("request_id: {request_id} - New Subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("request_id: {request_id} - /subscriptions: failed to execute query: {e:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
