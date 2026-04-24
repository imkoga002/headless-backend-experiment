use axum::{
    Json, Router,
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    github_user_id: String,
    username: String,
    created_at: DateTime<Utc>,
}

fn user_create(github_user_id: String, username: String) -> User {
    User {
        id: Uuid::new_v4().to_string(),
        github_user_id,
        username,
        created_at: Utc::now(),
    }
}

async fn github_login() -> impl IntoResponse {
    let client_id = std::env::var("GITHUB_CLIENT_ID").unwrap();
    let url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&scope=user",
        client_id
    );
    Redirect::to(&url)
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
}

#[derive(Deserialize)]
struct GitHubTokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct GitHubUser {
    id: u64,
    login: String,
}

async fn exchange_code_for_token(code: &str) -> String {
    let client = reqwest::Client::new();
    let res: GitHubTokenResponse = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .json(&serde_json::json!({
            "client_id": std::env::var("GITHUB_CLIENT_ID").unwrap(),
            "client_secret": std::env::var("GITHUB_CLIENT_SECRET").unwrap(),
            "code": code,
        }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    res.access_token
}

async fn fetch_github_user(token: &str) -> GitHubUser {
    let client = reqwest::Client::new();
    client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "rs-headless-backend")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

async fn github_callback(Query(query): Query<CallbackQuery>) -> impl IntoResponse {
    let token = exchange_code_for_token(&query.code).await;
    let github_user = fetch_github_user(&token).await;
    let user = user_create(github_user.id.to_string(), github_user.login);
    (StatusCode::CREATED, Json(user))
}

fn create_app() -> Router {
    Router::new()
        .route("/auth/github", get(github_login))
        .route("/auth/github/callback", get(github_callback))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, create_app()).await.unwrap();
}

// This test is using oneshot.
// #[cfg(test)]
// mod tests
// async fn test_user_create() {
//     let app = create_app();
//     let response = app
//         .oneshot(
//             Request::builder()
//                 .method("POST")
//                 .uri("/users")
//                 .header("content-type", "application/json")
//                 .body(Body::from(json!({"name": "Alice"}).to_string()))
//                 .unwrap(),
//         )
//         .await
//         .unwrap();
//     assert_eq!(response.status(), StatusCode::CREATED);
//     let body = axum ::body::to_bytes(response.into_body(), usize::MAX)
//         .await
//         .unwrap();
//     let user: Value = serde_json::from_slice(&body).unwrap();
//     assert_eq!(user["name"], "Alice");
//     assert!(user["id"].is_number());
// }

//this test doesnt require tower. just check logic without through http.
// #[tokio::test]
// async fn test_create_user() {
//     let payload = Json(CreateUserRequest { name: "Alice".to_string()});
//     let (status, Json(user)) = create_user(payloa).await;
//     assert_eq!(status, StatusCode::CREATED);
//     assert_eq!(user.name, "Alice");
// }
