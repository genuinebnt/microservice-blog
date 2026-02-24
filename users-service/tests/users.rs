mod common;

use common::{UserRequest, UserResponse};
use uuid::Uuid;

fn sample_user() -> UserRequest {
    let id = Uuid::new_v4();
    UserRequest {
        username: format!("user_{}", id),
        email: format!("{}@example.com", id),
    }
}

#[tokio::test]
async fn create_user_returns_200_for_valid_data() {
    let app = common::spawn_app().await;
    let user = sample_user();

    let response = app.post_user(&user).await;
    assert_eq!(response.status(), 200);

    let created: UserResponse = response.json().await.unwrap();
    assert_eq!(created.username, user.username);
    assert_eq!(created.email, user.email);

    let get_response = app.get_user_by_id(created.id).await;
    assert_eq!(get_response.status(), 200);

    let fetched: UserResponse = get_response.json().await.unwrap();
    assert_eq!(fetched.username, user.username);
    assert_eq!(fetched.email, user.email);
}

#[tokio::test]
async fn create_user_returns_422_for_missing_fields() {
    let app = common::spawn_app().await;

    let response = app
        .api_client
        .post(format!("http://{}/users", app.address))
        .json(&serde_json::json!({"username": "only_username"}))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 422);
}

#[tokio::test]
async fn get_user_returns_404_for_nonexistent_user() {
    let app = common::spawn_app().await;

    let response = app.get_user_by_id(Uuid::new_v4()).await;
    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn list_users_returns_all_created_users() {
    let app = common::spawn_app().await;

    let users = vec![sample_user(), sample_user(), sample_user()];

    for user in &users {
        let response = app.post_user(user).await;
        assert!(response.status().is_success());
    }

    let response = app.list_users().await;
    assert_eq!(response.status(), 200);

    // users-service returns PaginatedResponse
    let listed: serde_json::Value = response.json().await.unwrap();
    let data = listed.get("data").unwrap().as_array().unwrap();
    assert!(data.len() >= 3);
}

#[tokio::test]
async fn update_user_persists_changes() {
    let app = common::spawn_app().await;
    let user = sample_user();

    let created: UserResponse = app.post_user(&user).await.json().await.unwrap();

    let update_body = serde_json::json!({
        "id": created.id,
        "username": "updated_name",
        "email": "updated@example.com",
        "created_at": created.created_at,
        "updated_at": chrono::Utc::now(),
    });

    let response = app.update_user(created.id, &update_body).await;
    assert_eq!(response.status(), 200);

    let updated: UserResponse = app.get_user_by_id(created.id).await.json().await.unwrap();
    assert_eq!(updated.username, "updated_name");
    assert_eq!(updated.email, "updated@example.com");
}

#[tokio::test]
async fn delete_user_removes_user() {
    let app = common::spawn_app().await;
    let user = sample_user();

    let created: UserResponse = app.post_user(&user).await.json().await.unwrap();

    let response = app.get_user_by_id(created.id).await;
    assert_eq!(response.status(), 200);

    let response = app.delete_user(created.id).await;
    assert_eq!(response.status(), 200);

    let response = app.get_user_by_id(created.id).await;
    assert_eq!(response.status(), 404);
}
