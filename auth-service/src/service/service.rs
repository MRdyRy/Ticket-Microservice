use crate::domain::auth_domain::Request;
use crate::repository::http::HttpProvider;
use crate::repository::redis::CacheProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct Service {
    pub cache: Arc<dyn CacheProvider>,
    pub http: Arc<dyn HttpProvider>,
}

pub trait AuthService {
    async fn generate_token(&self, req: Request, s: String) -> Result<Option<String>, anyhow::Error>;
}

impl Service {
    pub fn new(cache: Arc<dyn CacheProvider>, http: Arc<dyn HttpProvider>) -> Service {
        Service { cache, http }
    }
}
