use crate::domain::authdomain::{Request, UserResponse};
use axum::async_trait;
use dyn_clone::DynClone;
use reqwest::{Client, ClientBuilder};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Http {
    pub client: Client,
    pub url_user: String,
}

#[async_trait]
pub trait AuthProvider {
    async fn check_user(&self, req: Request) -> Result<bool, Box<dyn Error>>;
}

#[derive(Debug, Clone)]
pub struct UserRepository {
    pub url: String,
}

#[async_trait]
pub trait HttpProvider: AuthProvider + Send + Sync + DynClone {}

impl Http {
    pub fn new(user: UserRepository) -> Http {
        let s = ClientBuilder::new().build().unwrap();
        Http {
            client: s,
            url_user: user.url,
        }
    }
}

impl AuthProvider for Http {
    async fn check_user(&self, req: Request) -> Result<bool, Box<dyn Error>> {
        match self
            .client
            .post(self.url_user.parse().unwrap())
            .json(&req)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<UserResponse>().await {
                        Ok(isValid) => Ok(isValid.into()),
                        _ => Ok(false),
                    }
                } else {
                    Ok(false)
                }
            }
            Err(_) => Ok(false),
        }
    }
}
