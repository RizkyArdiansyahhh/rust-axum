use std::fmt::format;

use axum::{Json, Router, extract::{Path, Query}, routing::{Route, get, post}};
use serde::Deserialize;
use tokio::net::TcpListener;




async fn get_jakarta() -> String{
    String::from("Saya ke jakarta")


async fn get_bogor()-> String{
    String::from("Saya ke bogor")
}

async fn get_indonesia()-> String{
    String::from("Saya ke indonesia")
}


#[derive(Deserialize)]
struct Pagination {
    size: u32,
    page: u32,
}

#[derive(Deserialize)]
struct User {
    id: u32,
    name: String,
}


async fn get_user(
    Path(id): Path<String>
) -> String {
    format!("user id {}", id)
}

async fn get_users_pagination(
    Query(
        payload
    ) : Query<Pagination>
) -> String {
    format!("Page {} dengan limit {}", payload.size , payload.page)
}


async fn create_user(
    Json(payload): Json<User>
)-> String{
    format!("user name {}", payload.name)
}


#[tokio::main]
async fn main() {

    // let daerah_routes = Router::new()
    // .route("/", get(get_indonesia))
    // .route("/bogor", get(get_bogor))
    // .route("/jakarta", get(get_jakarta));

    let user_router = Router::new()
    .route("/{id}", get(get_user))
    .route("/", get(get_users_pagination))
    .route("/", post(create_user));


    let app = Router::new()
    .nest("/users", user_router);


    let listener = TcpListener::bind("0.0.0.0:4000")
        .await
        .unwrap();

    axum::serve(listener, app)
    .await
    .unwrap();
}
