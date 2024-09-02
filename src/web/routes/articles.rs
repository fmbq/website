use crate::db::Pool;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Path},
    IntoResponse, Response,
};

#[handler]
pub async fn get(Data(db): Data<&Pool>, Path(id): Path<String>) -> Response {
    let mut conn = db.acquire().await.unwrap();

    let Some(image) = crate::db::articles::get_image_by_id(&mut conn, &id)
        .await
        .unwrap()
    else {
        return StatusCode::NOT_FOUND.into_response();
    };

    Response::builder()
        .content_type(image.content_type)
        .body(image.bytes)
}
