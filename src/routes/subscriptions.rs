use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::email_client::EmailClient;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "adding a new subscriber", 
    skip(form, pool, email_client), 
    fields( 
        subscriber_email = %form.email, 
        subscriber_name = %form.name
    )
)]

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>, email_client: web::Data<EmailClient>) -> impl Responder {

    let new_subscriber = match form.0.try_into(){
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    if insert_subscriber(&pool, &new_subscriber).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let confirmation_link = "https://there-s-no-such-domain.com/subscriptions/confirm";

    if email_client.send_email(
        new_subscriber.email, "welcome!",
        &format!("welcome to our newsletter! <br /> Click <a href=\"{}\"here</a> to confirm your subscription", confirmation_link),
        &format!("welcome to our newsletter! visit {} to confirm your subscription", confirmation_link)
    ).await.is_err() {
        return HttpResponse::InternalServerError().finish()
    }

    HttpResponse::Ok().finish()

}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
    let name = SubscriberName::parse(value.name)?;
    let email = SubscriberEmail::parse(value.email)?;

    Ok(NewSubscriber {email, name})
    }
}


#[tracing::instrument(
    name = "saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]

pub async fn insert_subscriber(pool: &PgPool, new_subscriber: &NewSubscriber) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions(id, email, name, subscribed_at, status)
        VALUES($1,$2, $3, $4, 'confirmed')
    "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {:?}", e);
        e
    })?;
    Ok(())
}
