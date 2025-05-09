use crate::{
    db::Pool, domain::auth, services::email::Mailer,
    web::components::admin::password_reset::request_password_reset_accepted,
};
use maud::Markup;
use poem::{
    handler,
    web::{Data, Form, Html},
    IntoResponse, Response,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestPasswordResetForm {
    email: String,
}

#[handler]
pub fn get() -> Markup {
    crate::web::pages::login::request_password_reset()
}

#[handler]
pub async fn submit(
    Data(db): Data<&Pool>,
    Data(mailer): Data<&Mailer>,
    Form(f): Form<RequestPasswordResetForm>,
) -> Response {
    let mut conn = db.acquire().await.unwrap();

    auth::request_password_reset(&mut conn, mailer, &f.email)
        .await
        .unwrap();

    Html(request_password_reset_accepted()).into_response()
}
