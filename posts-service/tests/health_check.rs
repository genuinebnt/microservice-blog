mod common;

#[tokio::test]
async fn health_check() {
    let test_app = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}{}", test_app.address, "/healthz"))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
}
