use serde::{Serialize, Deserialize};
use warp::{Filter};

#[derive(Serialize, Deserialize)]
struct User {
    uid: String,
    username: String,
    email: String,
    pw: String,
    date_register: i64,  // дата регистрации в формате секунд с 01.01.1970
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    email: String,
    pw: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let hello_route = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello_route)
        .run(([127, 0, 0, 1], 8000))
        .await
}
