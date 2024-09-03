use crate::db::{
    self,
    articles::{ArticleSummary, SortOrder},
    Connection,
};
use serde::{Deserialize, Serialize};
use sqlx::Acquire;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArticleContents {
    time: u64,
    version: String,
    blocks: Vec<BlockWithId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockWithId {
    id: String,

    #[serde(flatten)]
    block: Block,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
pub enum Block {
    Header { text: String, level: u8 },
    Paragraph { text: String },
}

/// Get a list of all articles.
pub async fn list_articles(connection: &mut Connection) -> Vec<ArticleSummary> {
    db::articles::list_after(connection, SortOrder::CreatedTime, 50)
        .await
        .unwrap()
}

/// Get the contents of an article by its ID.
pub async fn get_by_id(_connection: &mut Connection, _id: &str) -> Option<ArticleContents> {
    todo!()
}

/// Create a new empty article and return its ID. The article will be untitled.
pub async fn create(connection: &mut Connection) -> String {
    db::articles::create(connection, "Untitled article")
        .await
        .unwrap()
}

/// Request to update an existing article.
pub struct UpdateArticle {
    /// The ID of the article to update.
    id: String,

    /// If present, replaces the article title with the given string.
    title: Option<String>,

    /// If present, the new article text contents in Markdown. Overwrites the
    /// previous contents.
    markdown: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum UpdateArticleError {
    #[error("article not found with ID `{0}`")]
    ArticleNotFound(String),

    #[error("unknown database error")]
    Sql(#[from] sqlx::Error),
}

/// Update an existing article according to the given new values.
pub async fn update_article(
    connection: &mut Connection,
    update: UpdateArticle,
) -> Result<(), UpdateArticleError> {
    let mut tx = connection.begin().await?;

    let Some(_article) = db::articles::get_by_id(&mut tx, &update.id).await? else {
        return Err(UpdateArticleError::ArticleNotFound(update.id));
    };

    // TODO: Who has permission to update an article?

    if update.title.is_none() && update.markdown.is_none() {
        // Nothing to do.
        return Ok(());
    }

    db::articles::update(
        &mut tx,
        &update.id,
        update.title.as_deref(),
        update.markdown.as_deref(),
    )
    .await?;

    tx.commit().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_article_contents() {
        let json = r#"{
            "time": 1618033988,
            "version": "2.29.1",
            "blocks": [
                {
                    "id": "block-1",
                    "type": "header",
                    "data": {
                        "text": "Hello, world!",
                        "level": 1
                    }
                },
                {
                    "id": "block-2",
                    "type": "paragraph",
                    "data": {
                        "text": "This is a test article."
                    }
                }
            ]
        }"#;

        let contents: ArticleContents = serde_json::from_str(json).unwrap();
        assert_eq!(contents.time, 1618033988);
        assert_eq!(contents.version, "2.29.1");
        assert_eq!(contents.blocks.len(), 2);

        let block1 = &contents.blocks[0];
        assert_eq!(block1.id, "block-1");
        assert!(matches!(&block1.block, Block::Header { .. }));

        let block2 = &contents.blocks[1];
        assert_eq!(block2.id, "block-2");
        assert!(matches!(&block2.block, Block::Paragraph { .. }));
    }
}
