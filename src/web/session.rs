use poem::{http::StatusCode, session::Session, Error, FromRequest, Request, RequestBody, Result};

pub struct LoginSession {
    pub user_id: String,
}

impl<'a> FromRequest<'a> for LoginSession {
    async fn from_request(request: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let session = request.data::<Session>().unwrap();

        if let Some(user_id) = session.get::<String>("user-id") {
            Ok(LoginSession { user_id })
        } else {
            Err(Error::from_string(
                "not logged in",
                StatusCode::UNAUTHORIZED,
            ))
        }
    }
}
