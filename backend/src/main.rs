use axum::extract::MatchedPath;
use axum::http::Request;
use axum::response::Html;
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::info_span;

#[tokio::main]
async fn main() {
    backend::trace_init();

    let app = Router::new()
        .route("/", get(hello))
        .layer(TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            let matched_path = request.extensions().get::<MatchedPath>().map(MatchedPath::as_str);

            info_span!("http_request", method = ?request.method(), matched_path, some_other_field = tracing::field::Empty)
        }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
