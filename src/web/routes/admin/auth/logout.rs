use poem::{handler, session::Session, web::Redirect, IntoResponse, Response};

#[handler]
pub async fn get(session: &Session) -> Response {
    session.purge();

    Redirect::see_other("/").into_response()
}
