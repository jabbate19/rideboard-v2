use anyhow::Result;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::json;

struct PingClient {
    client: Client,
    join_route: String,
    leave_route: String,
}

impl PingClient {
    pub fn new(token: &str, join_route: &str, leave_route: &str) -> Result<PingClient> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {token}"))?,
        );

        let client = Client::builder().default_headers(headers).build()?;

        Ok(PingClient {
            client,
            join_route: join_route.to_string(),
            leave_route: leave_route.to_string(),
        })
    }

    pub async fn send_join(&self, to: &str, join: &str, ride: &str) -> Result<()> {
        self.client
            .post(format!(
                "https://pings.csh.rit.edu/service/route/{}/ping",
                self.join_route
            ))
            .json(&json!({
                "username": to,
                "body": format!("@{join} has joined \"{ride}\"")
            }))
            .send()
            .await?;
        Ok(())
    }

    pub async fn send_leave(&self, to: &str, leave: &str, ride: &str) -> Result<()> {
        self.client
            .post(format!(
                "https://pings.csh.rit.edu/service/route/{}/ping",
                self.leave_route
            ))
            .json(&json!({
                "username": to,
                "body": format!("@{leave} has leaved \"{ride}\"")
            }))
            .send()
            .await?;
        Ok(())
    }
}
