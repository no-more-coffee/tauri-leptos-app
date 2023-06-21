use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use hyper::{Body, Response};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use urlencoding::decode;

pub async fn main() {
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let app = Router::new().route("/files/*file_path", get(serve_file));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn serve_file(Path(file_path): Path<String>) -> impl IntoResponse {
    let decoded_path = format!("/{}", decode(&file_path).unwrap());
    let file = File::open(decoded_path).await.unwrap();
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    let response = Response::new(body);
    response
}
