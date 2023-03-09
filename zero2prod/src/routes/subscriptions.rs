use actix_web::{HttpResponse, web};
use sqlx::{PgPool};
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct SubscriptionFormData {
    name: String,
    email: String,
}

#[tracing::instrument(
name = "POST subscriptions"
skip(body, conn)
fields(
subscriber_name = % body.name,
subscriber_email = % body.email
)
)]
pub async fn subscriptions(body: web::Form<SubscriptionFormData>, conn: web::Data<PgPool>)
                           -> HttpResponse {
    match db_insert_subscriber(&body, &conn).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(skip(body, conn))]
async fn db_insert_subscriber(body: &SubscriptionFormData, conn: &PgPool)
                              -> Result<(), sqlx::Error> {
    let at = Utc::now();
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        id, body.email, body.name, at
    )
        .execute(conn)
        .await
        .map_err(|e| {
            tracing::error!("failed to execute query: {e:?}");
            e
        })?;
    Ok(())
}