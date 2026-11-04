use axum::{Router, routing::get};
use tokio::net::TcpListener;




async fn get_jakarta() -> String{
    String::from("Saya ke jakarta")
}

async fn get_bogor()-> String{
    String::from("Saya ke bogor")
}

async fn get_indonesia()-> String{
    String::from("Saya ke indonesia")
}

#[tokio::main]
async fn main() {

    let daerah_routes = Router::new()
    .route("/", get(get_indonesia))
    .route("/bogor", get(get_bogor))
    .route("/jakarta", get(get_jakarta));

    let app  = Router::new()
   .nest("/indonesia", daerah_routes);


    let listener = TcpListener::bind("0.0.0.0:4000")
        .await
        .unwrap();

    axum::serve(listener, app)
    .await
    .unwrap();
}
