use actix_web::{HttpResponse, web};
use sqlx::{PgPool};
use uuid::Uuid;
use chrono::Utc;
use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

#[derive(serde::Deserialize)]
pub struct SubscriptionFormData {
    name: String,
    email: String,
}

impl TryFrom<SubscriptionFormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: SubscriptionFormData) -> Result<Self, Self::Error> {
        Ok(NewSubscriber {
            name: SubscriberName::parse(value.name)?,
            email: SubscriberEmail::parse(value.email)?,
        })
    }
}

#[tracing::instrument(
name = "POST subscriptions"
skip(form_data, conn)
fields(subscriber_name = % form_data.name,subscriber_email = % form_data.email)
)]
pub async fn subscriptions(form_data: web::Form<SubscriptionFormData>, conn: web::Data<PgPool>)
                           -> HttpResponse {
    let new_subscriber = match form_data.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => {
            return HttpResponse::BadRequest().finish();
        }
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
        id, new_subscriber.email.as_ref(), new_subscriber.name.as_ref(), at)
        .execute(conn)
        .await
        .map_err(|e| {
            tracing::error!("failed to execute query: {e:?}");
            e
        })?;
    Ok(())
}
