use reqwest::Client;
use std::process::Command;
use std::time::Duration;

pub struct TestGateway {
    pub address: String,
    pub client: Client,
}

impl TestGateway {
    pub fn new() -> Self {
        let address = std::env::var("GATEWAY_BASE_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:9000".to_string());

        Self {
            address,
            client: Client::new(),
        }
    }

    pub async fn wait_until_ready(&self) {
        let health_url = format!("{}/health_check", self.address);
        for i in 0..30 {
            if let Ok(res) = self.client.get(&health_url).send().await
                && res.status().is_success()
            {
                return;
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
            if i % 5 == 0 {
                println!("Waiting for gateway at {}...", health_url);
            }
        }
        panic!("Gateway never became ready at {}", self.address);
    }

    pub fn docker_compose_up(profile: Option<&str>) {
        let mut cmd = Command::new("docker");
        cmd.arg("compose")
            .arg("--profile")
            .arg(profile.unwrap_or("test"))
            .arg("up")
            .arg("-d")
            .arg("--build");

        let status = cmd.status().expect("Failed to execute docker compose");
        assert!(status.success(), "docker compose up failed");
    }

    pub fn docker_compose_down(profile: Option<&str>) {
        let mut cmd = Command::new("docker");
        cmd.arg("compose")
            .arg("--profile")
            .arg(profile.unwrap_or("test"))
            .arg("down")
            .arg("-v");

        let status = cmd.status().expect("Failed to execute docker compose down");
        assert!(status.success(), "docker compose down failed");
    }
}
