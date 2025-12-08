mod common;

#[tokio::test]
pub async fn create_post_returns_200_for_valid_data() {
    let test_app = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("http://{}{}", test_app.address, "/posts"))
        .json(&serde_json::json!({
            "title": "Test Post",
            "content": "Test Content",
        }))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
}
