use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "adding a new subscriber", 
    skip(form, pool), 
    fields( 
        subscriber_email = %form.email, 
        subscriber_name = %form.name
    )
)]


pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => {
            tracing::info!(" New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(" Failed to exectue query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(
    name = "saving new subscriber details in the database", skip(form, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool, form : &FormData
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES($1,$2, $3, $4)

    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await.map_err(|e| {
            tracing::error!("Failed to execute query {:?}", e);
            e
        })?;
    Ok(())
}
