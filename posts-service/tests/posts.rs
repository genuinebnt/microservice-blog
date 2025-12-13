mod common;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreatePostResponse {
    pub id: uuid::Uuid,
}

#[derive(Serialize, Debug)]
pub struct PostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ListPostResponse {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct GetPostResponse {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[tokio::test]
pub async fn create_post_returns_200_for_valid_data() {
    let test_app = common::spawn_app().await;
    let post = PostRequest {
        title: "Test Post".to_string(),
        content: "Test Content".to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("http://{}/posts", test_app.address))
        .json(&post)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let saved_post: CreatePostResponse = response.json().await.unwrap();

    let post_in_db = test_app
        .repo_provider
        .posts
        .get(saved_post.id)
        .await
        .unwrap();

    assert!(post_in_db.is_some());
    let post_in_db = post_in_db.unwrap();
    assert_eq!(post_in_db.title, "Test Post");
    assert_eq!(post_in_db.content, "Test Content");
}

#[tokio::test]
pub async fn list_posts_returns_valid_list_for_valid_data() {
    let test_app = common::spawn_app().await;

    let posts = vec![
        PostRequest {
            title: "Test Post 1".to_string(),
            content: "Test Content 1".to_string(),
        },
        PostRequest {
            title: "Test Post 2".to_string(),
            content: "Test Content 2".to_string(),
        },
    ];

    let client = reqwest::Client::new();

    for post in posts {
        let response = client
            .post(&format!("http://{}/posts", test_app.address))
            .json(&post)
            .send()
            .await
            .unwrap();

        assert!(response.status().is_success());
    }

    let response = client
        .get(&format!("http://{}/posts", test_app.address))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    let posts = response.json::<Vec<ListPostResponse>>().await.unwrap();
    assert_eq!(posts.len(), 2);
}

#[tokio::test]
pub async fn get_post_returns_200_for_valid_data() {
    let test_app = common::spawn_app().await;
    let post = PostRequest {
        title: "Test Post".to_string(),
        content: "Test Content".to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("http://{}/posts", test_app.address))
        .json(&post)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let saved_post: CreatePostResponse = response.json().await.unwrap();

    let post_in_db = test_app
        .repo_provider
        .posts
        .get(saved_post.id)
        .await
        .unwrap();

    assert!(post_in_db.is_some());
    let post_in_db = post_in_db.unwrap();
    assert_eq!(post_in_db.title, "Test Post");
    assert_eq!(post_in_db.content, "Test Content");

    let response = client
        .get(&format!(
            "http://{}/posts/{}",
            test_app.address, saved_post.id
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    let post = response.json::<GetPostResponse>().await.unwrap();
    assert_eq!(post.id, saved_post.id);
    assert_eq!(post.title, "Test Post");
    assert_eq!(post.content, "Test Content");
}

#[tokio::test]
pub async fn update_post_returns_200_for_valid_data() {
    let test_app = common::spawn_app().await;
    let post = PostRequest {
        title: "Test Post".to_string(),
        content: "Test Content".to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("http://{}/posts", test_app.address))
        .json(&post)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let saved_post: CreatePostResponse = response.json().await.unwrap();

    let post_in_db = test_app
        .repo_provider
        .posts
        .get(saved_post.id)
        .await
        .unwrap();

    assert!(post_in_db.is_some());
    let mut post_in_db = post_in_db.unwrap();
    assert_eq!(post_in_db.title, "Test Post");
    assert_eq!(post_in_db.content, "Test Content");

    post_in_db.title = "Updated Test Post".to_string();
    post_in_db.content = "Updated Test Content".to_string();

    let response = client
        .put(&format!(
            "http://{}/posts/{}",
            test_app.address, saved_post.id
        ))
        .json(&post_in_db)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let updated_post = client
        .get(&format!(
            "http://{}/posts/{}",
            test_app.address, saved_post.id
        ))
        .send()
        .await
        .unwrap()
        .json::<GetPostResponse>()
        .await
        .unwrap();

    assert_eq!(updated_post.id, saved_post.id);
    assert_eq!(updated_post.title, "Updated Test Post");
    assert_eq!(updated_post.content, "Updated Test Content");
}
