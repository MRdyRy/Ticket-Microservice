use crate::domain::auth_domain::{Request, UserResponse};
use axum::async_trait;
use reqwest::{Client, ClientBuilder};

#[derive(Debug, Clone)]
pub struct RestRepository {
    pub client: Client,
    pub url_user: String,
}

#[async_trait]
pub trait AuthProvider {
    async fn check_user(&self, req: Request) -> Result<bool, anyhow::Error>;
}

#[derive(Debug, Clone)]
pub struct UserRepository {
    pub url: String,
}

#[async_trait]
pub trait HttpProvider: AuthProvider + Send + Sync {}

impl RestRepository {
    pub fn new(user: UserRepository) -> RestRepository {
        let client = ClientBuilder::new()
            .build()
            .expect("Failed to create HTTP client");
        RestRepository {
            client,
            url_user: user.url,
        }
    }
}

impl HttpProvider for RestRepository {
    
}
#[async_trait]
impl AuthProvider for RestRepository {
    async fn check_user(&self, req: Request) -> Result<bool, anyhow::Error> {
        // Send the POST request to check user
        let response = self
            .client
            .post(self.url_user.clone())
            .json(&req)
            .send()
            .await;

        match response {
            Ok(resp) if resp.status().is_success() => match resp.json::<UserResponse>().await {
                Ok(user_response) => Ok(user_response.valid),
                Err(err) => anyhow::bail!("HTTP error: {}", err),
            },
            Ok(_) => Ok(false),
            Err(err) => Err(anyhow::anyhow!(err).context("error validate user")),
        }
    }
}
