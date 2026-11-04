use axum::Router;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {

    let app  = Router::new();

    let listener = TcpListener::bind("0.0.0.0:4000")
        .await
        .unwrap();

    axum::serve(listener, app)
    .await
    .unwrap();
}
