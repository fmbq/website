use crate::db::{
    users::{get_by_id, User},
    Connection,
};

pub async fn get_profile(connection: &mut Connection, id: &str) -> sqlx::Result<Option<User>> {
    Ok(get_by_id(connection, id).await)
}
