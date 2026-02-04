use crate::helpers::TestGateway;
use serde_json::json;

#[tokio::test]
async fn gateway_aggregates_users_and_posts() {
    TestGateway::docker_compose_up(Some("test"));
    let gateway = TestGateway::new();
    gateway.wait_until_ready().await;

    let graphql_query = json!({
        "query": r#"
        query {
          posts {
            id
            title
            authorName
          }
        }
        "#
    });

    let response = gateway
        .client
        .post(format!("{}/graphql", gateway.address))
        .json(&graphql_query)
        .send()
        .await
        .expect("Failed to call gateway");

    assert!(response.status().is_success(), "Gateway request failed");

    let body: serde_json::Value = response.json().await.unwrap();

    assert!(body.get("data").is_some(), "No 'data' field in response");
    println!("body: {:#?}", body);
    assert!(body["data"].get("posts").is_some(), "Missing 'posts' field");
}
