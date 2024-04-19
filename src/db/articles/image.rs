use crate::db::Connection;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

/// An image file that may be embedded in an article.
///
/// Images are immutable and cannot be modified once uploaded. A new one must be
/// uploaded and the old one deleted to make changes.
#[derive(FromRow, Debug)]
pub struct Image {
    pub(crate) id: String,

    /// A media type indicating the format the image is encoded in. We convert on
    /// upload into specific formats, so in practice this should not vary much.
    pub(crate) content_type: String,

    /// Timestamp when the image was uploaded.
    pub(crate) uploaded_time: DateTime<Utc>,

    /// The byte contents of the image.
    pub(crate) bytes: Vec<u8>,
}

/// Create a new image.
pub async fn create_image(
    connection: &mut Connection,
    content_type: &str,
    bytes: Vec<u8>,
) -> Result<String, sqlx::Error> {
    let id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO image (id, content_type, uploaded_time, bytes) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(content_type)
        .bind(Utc::now())
        .bind(bytes)
        .execute(connection)
        .await?;

    Ok(id)
}

pub async fn get_image_by_id(
    connection: &mut Connection,
    id: &str,
) -> Result<Option<Image>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM article_images WHERE id = ?")
        .bind(id)
        .fetch_optional(connection)
        .await
}
