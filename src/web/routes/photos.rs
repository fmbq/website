//! Image optimization and serving for static images part of the web design.

use directories::ProjectDirs;
use image::DynamicImage;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Path as RequestPath},
    Body, Response,
};
use serde::Serialize;
use sha1::{Digest, Sha1};
use std::path::{Path, PathBuf};
use tokio::{fs::File, task::spawn_blocking};

use crate::config::Configuration;

#[handler]
pub async fn get_photo(
    Data(dirs): Data<&ProjectDirs>,
    Data(config): Data<&Configuration>,
    RequestPath(file_name): RequestPath<String>,
) -> Response {
    let wwwroot = std::path::Path::new("wwwroot/static/resources/photos");
    let image_path = file_name.trim_start_matches('/');
    let image_path = wwwroot.join(image_path);

    if config.dynamic_image_conversions {
        let content_type = "image/webp";
        let cache_path = cache_file_path(dirs.cache_dir(), &file_name, content_type);

        if !cache_path.exists() {
            let image_path = image_path.clone();
            let cache_path = cache_path.clone();

            let _result = spawn_blocking(move || {
                let image = load_image(image_path)?;
                let encoder = webp::Encoder::from_image(&image).unwrap();
                let encoded = encoder.encode(75.0);

                std::fs::create_dir_all(cache_path.parent().unwrap())?;
                std::fs::write(cache_path, &*encoded)?;

                Ok::<_, color_eyre::eyre::Error>(())
            })
            .await;
        }

        File::open(cache_path)
            .await
            .map(|file| {
                Response::builder()
                    .content_type(content_type)
                    .body(Body::from_async_read(file))
            })
            .unwrap_or_else(|e| {
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(format!("Image not found: {}", e))
            })
    } else {
        File::open(&image_path)
            .await
            .map(|file| {
                Response::builder()
                    .content_type(match image_path.extension().and_then(|s| s.to_str()) {
                        Some("webp") => "image/webp",
                        Some("png") => "image/png",
                        _ => "image/jpeg",
                    })
                    .body(Body::from_async_read(file))
            })
            .unwrap_or_else(|e| {
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(format!("Image not found: {}", e))
            })
    }
}

#[derive(Clone, Debug, Serialize)]
struct CacheKey {
    file_name: String,
    content_type: String,
}

fn cache_file_path(cache_dir: &Path, file_name: &str, content_type: &str) -> PathBuf {
    let cache_key = CacheKey {
        file_name: file_name.to_string(),
        content_type: content_type.to_string(),
    };

    let mut hasher = Sha1::new();
    serde_json::to_writer(&mut hasher, &cache_key).unwrap();
    let hash = hasher.finalize();
    let hash_string = hex::encode(hash);

    cache_dir.join("images").join(hash_string)
}

async fn load_image_async(path: impl AsRef<Path>) -> color_eyre::eyre::Result<DynamicImage> {
    let path = path.as_ref().to_path_buf();

    spawn_blocking(move || load_image(path)).await?
}

fn load_image(path: impl AsRef<Path>) -> color_eyre::eyre::Result<DynamicImage> {
    let img = image::ImageReader::open(path.as_ref())?.decode()?;

    Ok(img)
}
