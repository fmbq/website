use crate::db::{
    users::{get_by_id, User},
    Pool,
};
use poem::{http::StatusCode, session::Session, Error, FromRequest, Request, RequestBody, Result};

/// Provides information about the currently logged in user.
///
/// This struct acts as a proof of login, and can only be obtained if a user is
/// actually logged in.
pub struct LoginContext {
    pub user_id: String,
    pub user: User,
}

impl<'a> FromRequest<'a> for LoginContext {
    async fn from_request(request: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let session = request.data::<Session>().unwrap();

        if let Some(user_id) = session.get::<String>("user-id") {
            let db = request.data::<Pool>().unwrap();
            let mut conn = db.acquire().await.unwrap();
            let user = get_by_id(&mut conn, &user_id).await.unwrap();

            Ok(LoginContext { user_id, user })
        } else {
            Err(Error::from_string(
                "not logged in",
                StatusCode::UNAUTHORIZED,
            ))
        }
    }
}
