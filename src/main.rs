use axum::{Json, Router, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
}

#[derive(Debug, Deserialize)]
struct CreateUserInput {
    username: String,
}

fn user_create(username: String) -> User {
    User {
        id: "1".to_string(),
        username,
    }
}

async fn github_login() -> impl IntoResponse {
    let url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&scope=user",
        std::env::var("GITHUB_CLIENT_ID").unwrap()
    );
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
}

async fn github_callback(Query(query): Query<CallbackQUery>) -> impl IntoResponse {
    let token = exchange_code_for_token(&query.code).await;
    let github_user = fetch_github_user(&token).await;
    Json(json!({ "token": "..." }))
}

async fn create_user_handler(Json(input): Json<CreateUserInput>) -> (StatusCode, Json<User>) {
    let user = user_create(input.username);
    (StatusCode::CREATED, Json(user))
}

fn create_app() -> Router {
    Router::new().route("/users", post(create_user_handler))
}
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, create_app()).await.unwrap();
}

#[test]
fn test_user_create() {
    let user = user_create("Alice".to_string());
    assert_eq!(user.username, "Alice");
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
