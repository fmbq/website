use crate::db::Connection;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, QueryBuilder};
use uuid::Uuid;

mod image;

pub use self::image::*;

#[derive(FromRow, Debug)]
pub struct Article {
    pub(crate) id: String,
    pub(crate) slug: String,
    pub(crate) title: String,
    pub(crate) created_time: DateTime<Utc>,
    pub(crate) updated_time: DateTime<Utc>,
    pub(crate) publish_time: Option<DateTime<Utc>>,
    pub(crate) markdown: String,
}

/// A summary of an article, containing only the metadata.
#[derive(FromRow, Debug)]
pub struct ArticleSummary {
    pub(crate) id: String,
    pub(crate) slug: String,
    pub(crate) title: String,
    pub(crate) created_time: DateTime<Utc>,
    pub(crate) updated_time: DateTime<Utc>,
    pub(crate) publish_time: Option<DateTime<Utc>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortOrder {
    Title,
    CreatedTime,
    UpdatedTime,
    PublishTime,
}

/// Fetch a list of article summaries, optionally after another article.
///
/// The results are paginated and sorted by creation time.
pub async fn list_after(
    connection: &mut Connection,
    sort_order: SortOrder,
    limit: u16,
    // TODO: Add an optional `after` parameter.
) -> Result<Vec<ArticleSummary>, sqlx::Error> {
    let mut query = QueryBuilder::new(
        "
        SELECT id, slug, title, created_time, updated_time, publish_time
        FROM article ",
    );

    query
        .push(" ORDER BY ")
        .push(match sort_order {
            SortOrder::Title => "title",
            SortOrder::CreatedTime => "created_time",
            SortOrder::UpdatedTime => "modified_time",
            SortOrder::PublishTime => "publish_time",
        })
        .push(", id DESC");

    query.push(" LIMIT ").push_bind(limit);

    query.build_query_as().fetch_all(connection).await
}

pub async fn create(connection: &mut Connection, title: &str) -> Result<String, sqlx::Error> {
    let id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO article (id, title) VALUES (?, ?)")
        .bind(&id)
        .bind(title)
        .execute(connection)
        .await?;

    Ok(id)
}

pub async fn get_by_id(
    connection: &mut Connection,
    id: &str,
) -> Result<Option<Article>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM article WHERE id = ?")
        .bind(id)
        .fetch_optional(connection)
        .await
}

pub async fn update(
    connection: &mut Connection,
    id: &str,
    title: Option<&str>,
    markdown: Option<&str>,
) -> Result<(), sqlx::Error> {
    let mut query = QueryBuilder::new("UPDATE article SET");

    if let Some(title) = title {
        query.push(" title = ").push_bind(title).push(",");
    }

    if let Some(markdown) = markdown {
        query.push(" markdown = ").push_bind(markdown).push(",");
    }

    query
        .push(" updated_time = now()")
        .push(" WHERE id = ")
        .push_bind(id);

    query.build().execute(connection).await?;

    Ok(())
}
