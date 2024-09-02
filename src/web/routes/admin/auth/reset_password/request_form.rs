use crate::{
    web::components::admin::password_reset::request_password_reset_accepted, db::Pool, domain::users,
    services::email::Mailer,
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
pub fn get() -> Html<Markup> {
    Html(crate::web::pages::login::request_password_reset())
}

#[handler]
pub async fn submit(
    Data(db): Data<&Pool>,
    Data(mailer): Data<&Mailer>,
    Form(f): Form<RequestPasswordResetForm>,
) -> Response {
    let mut conn = db.acquire().await.unwrap();

    users::request_password_reset(&mut conn, mailer, &f.email)
        .await
        .unwrap();

    Html(request_password_reset_accepted()).into_response()
}
