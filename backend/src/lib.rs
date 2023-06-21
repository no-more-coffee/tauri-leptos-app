use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use urlencoding::decode;

pub async fn main() {
    let app = Router::new().route("/files/*file_path", get(serve_file));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn serve_file(Path(path_param): Path<String>) -> Response {
    let decoded_path = match decode(&path_param) {
        Ok(s) => format!("/{}", s),
        Err(e) => return (StatusCode::BAD_REQUEST, format!("{}. Path param: {}", e, path_param)).into_response(),
    };
    let file = match File::open(&decoded_path).await {
        Ok(f) => f,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("{}. Path: {}", e, decoded_path)).into_response(),
    };
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    let response = Response::new(body);
    response.into_response()
}
