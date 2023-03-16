use actix_web::{HttpResponse, web};
use sqlx::{PgPool};
use uuid::Uuid;
use chrono::Utc;
use crate::domain::{NewSubscriber, SubscriberName};

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
    let subscriber_name = SubscriberName::parse(body.0.name);
    if subscriber_name.is_err() {
        return HttpResponse::BadRequest().finish();
    }
    let new_subscriber = NewSubscriber {
        name: subscriber_name.unwrap(),
        email: body.0.email,
    };
    match db_insert_subscriber(&new_subscriber, &conn).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(skip(new_subscriber, conn))]
async fn db_insert_subscriber(new_subscriber: &NewSubscriber, conn: &PgPool)
                              -> Result<(), sqlx::Error> {
    let at = Utc::now();
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        id, new_subscriber.email, new_subscriber.name.as_ref(), at)
        .execute(conn)
        .await
        .map_err(|e| {
            tracing::error!("failed to execute query: {e:?}");
            e
        })?;
    Ok(())
}
