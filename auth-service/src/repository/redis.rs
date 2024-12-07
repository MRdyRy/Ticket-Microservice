use crate::repository::http::AuthProvider;
use axum::async_trait;
use dyn_clone::DynClone;
use redis::{Client, Commands, RedisError, RedisResult};

#[derive(Clone, Debug)]
pub struct RedisRepository {
    client: Client,
}

#[async_trait]
pub trait CacheProvider {
    async fn set(&self, key: &str, value: &str) -> Result<bool, RedisError>;
    fn get(&self, key: &str) -> Result<String, RedisError>;
}
#[async_trait]
pub trait RedisProvider: CacheProvider + Send + Sync + DynClone {}

impl<T: RedisProvider + Send + Sync + DynClone> RedisProvider for T {}

impl RedisRepository {
    pub fn new(host: &str, password: &str) -> RedisRepository {
        let redis_host = format!("{}://:{}@{}", "redis", host, password);
        let client = redis::Client::open(redis_host).expect("Could not connect to Redis");
        RedisRepository { client }
    }
}

#[async_trait]
impl CacheProvider for RedisRepository {
    async fn set(&mut self, key: &str, value: &str) -> Result<bool, RedisError> {
        let result = self.client.set_ex(key, value, 3600).await?;
        Ok(result)
    }

    fn get(&mut self, key: &str) -> Result<String, RedisError> {
        let result = self.client.get(key)?;
        match result {
            RedisResult::Ok(val) => Ok(val),
        }
    }
}

