CREATE TABLE article (
    id TEXT NOT NULL PRIMARY KEY,
    slug TEXT NOT NULL,
    created_time TEXT NOT NULL,
    updated_time TEXT,
    publish_time TEXT,
    title TEXT NOT NULL,
    html TEXT NOT NULL
);

CREATE TABLE article_image (
    article_image_id TEXT NOT NULL PRIMARY KEY,
    uploaded_time TEXT NOT NULL,
    bytes BLOB NOT NULL
);

