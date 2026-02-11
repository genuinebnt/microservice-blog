mod common;

use common::{CreatePostResponse, GetPostResponse, ListPostResponse, PostRequest};

fn sample_post() -> PostRequest {
    PostRequest {
        title: "Test Post".to_string(),
        author_id: uuid::Uuid::new_v4(),
        content: "Test Content".to_string(),
    }
}

#[tokio::test]
async fn create_post_returns_200_for_valid_data() {
    let app = common::spawn_app().await;
    let post = sample_post();

    let response = app.post_post(&post).await;
    assert_eq!(response.status(), 200);

    let created: CreatePostResponse = response.json().await.unwrap();

    let get_response = app.get_post(created.id).await;
    assert_eq!(get_response.status(), 200);

    let fetched: GetPostResponse = get_response.json().await.unwrap();
    assert_eq!(fetched.title, post.title);
    assert_eq!(fetched.content, post.content);
}

#[tokio::test]
async fn create_post_returns_422_for_missing_fields() {
    let app = common::spawn_app().await;

    let response = app
        .api_client
        .post(format!("http://{}/posts", app.address))
        .json(&serde_json::json!({"title": "Only a title"}))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 422);
}

#[tokio::test]
async fn get_post_returns_200_for_existing_post() {
    let app = common::spawn_app().await;
    let post = sample_post();

    let created: CreatePostResponse = app.post_post(&post).await.json().await.unwrap();

    let response = app.get_post(created.id).await;
    assert_eq!(response.status(), 200);

    let fetched: GetPostResponse = response.json().await.unwrap();
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.title, "Test Post");
    assert_eq!(fetched.content, "Test Content");
}

#[tokio::test]
async fn get_post_returns_404_for_nonexistent_post() {
    let app = common::spawn_app().await;

    let response = app.get_post(uuid::Uuid::new_v4()).await;
    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn list_posts_returns_empty_list_when_no_posts() {
    let app = common::spawn_app().await;

    let response = app.list_posts().await;
    assert_eq!(response.status(), 200);

    let posts: Vec<ListPostResponse> = response.json().await.unwrap();
    assert!(posts.is_empty());
}

#[tokio::test]
async fn list_posts_returns_all_created_posts() {
    let app = common::spawn_app().await;

    let posts = vec![
        PostRequest {
            title: "Post 1".to_string(),
            author_id: uuid::Uuid::new_v4(),
            content: "Content 1".to_string(),
        },
        PostRequest {
            title: "Post 2".to_string(),
            author_id: uuid::Uuid::new_v4(),
            content: "Content 2".to_string(),
        },
        PostRequest {
            title: "Post 3".to_string(),
            author_id: uuid::Uuid::new_v4(),
            content: "Content 3".to_string(),
        },
    ];

    for post in &posts {
        let response = app.post_post(post).await;
        assert!(response.status().is_success());
    }

    let response = app.list_posts().await;
    assert_eq!(response.status(), 200);

    let listed: Vec<ListPostResponse> = response.json().await.unwrap();
    assert_eq!(listed.len(), 3);
}

#[tokio::test]
async fn update_post_persists_changes() {
    let app = common::spawn_app().await;
    let post = sample_post();

    let created: CreatePostResponse = app.post_post(&post).await.json().await.unwrap();

    // Fetch the full post so we have all fields for the PUT body
    let original: GetPostResponse = app.get_post(created.id).await.json().await.unwrap();

    let update_body = serde_json::json!({
        "id": original.id,
        "title": "Updated Title",
        "author_id": uuid::Uuid::new_v4(),
        "content": "Updated Content",
        "created_at": original.created_at,
        "updated_at": chrono::Utc::now(),
    });

    let response = app.update_post(created.id, &update_body).await;
    assert_eq!(response.status(), 200);

    let updated: GetPostResponse = app.get_post(created.id).await.json().await.unwrap();
    assert_eq!(updated.title, "Updated Title");
    assert_eq!(updated.content, "Updated Content");
}

#[tokio::test]
async fn delete_post_removes_post() {
    let app = common::spawn_app().await;
    let post = sample_post();

    let created: CreatePostResponse = app.post_post(&post).await.json().await.unwrap();

    let response = app.get_post(created.id).await;
    assert_eq!(response.status(), 200);

    let response = app.delete_post(created.id).await;
    assert_eq!(response.status(), 200);

    let response = app.get_post(created.id).await;
    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn get_post_twice_returns_same_data() {
    let app = common::spawn_app().await;
    let post = sample_post();

    let created: CreatePostResponse = app.post_post(&post).await.json().await.unwrap();

    let first: GetPostResponse = app.get_post(created.id).await.json().await.unwrap();
    let second: GetPostResponse = app.get_post(created.id).await.json().await.unwrap();

    assert_eq!(first.id, second.id);
    assert_eq!(first.title, second.title);
    assert_eq!(first.content, second.content);
}
