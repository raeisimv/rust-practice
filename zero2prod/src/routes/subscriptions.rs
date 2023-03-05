use actix_web::{HttpResponse, web};
use sqlx::{PgPool};
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct SubscriptionFormData {
    name: String,
    email: String,
}

pub async fn subscriptions(body: web::Form<SubscriptionFormData>, conn: web::Data<PgPool>) -> HttpResponse {
    log::info!("Adding '{}', '{}' as a new subscriber", body.name, body.email);

    let at = Utc::now();
    let id = Uuid::new_v4();

    log::info!("Saving subscriber details into the database");
    let res = sqlx::query!(r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
        id, body.email, body.name, at
    )
        .execute(conn.get_ref())
        .await
        ;
    match res {
        Ok(_) => {
            log::info!("New Subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("/subscriptions: failed to execute query: {e:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
