use axum::async_trait;
use dyn_clone::DynClone;
use redis::{AsyncCommands, Client, RedisError, RedisResult};

#[derive(Clone, Debug)]
pub struct RedisRepository {
    client: Client,
}

#[async_trait]
pub trait CacheProvider: Send + Sync {
    async fn set(&self, key: &str, value: &str) -> Result<bool, RedisError>;
    async fn get(&self, key: &str) -> Result<String, RedisError>;
}

#[async_trait]
pub trait RedisProvider: CacheProvider + Send + Sync + DynClone {}

impl<T: RedisProvider + Send + Sync + DynClone> RedisProvider for T {}

impl RedisRepository {
    pub fn new(host: &str, password: &str) -> RedisRepository {
        let redis_host = format!("redis://:{}@{}", password, host);
        let client = redis::Client::open(redis_host)
            .expect("Failed to connect to Redis");
        RedisRepository { client }
    }
}

#[async_trait]
impl CacheProvider for RedisRepository {
    async fn set(&self, key: &str, value: &str) -> Result<bool, RedisError> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let result: RedisResult<()> = con.set_ex(key, value, 3600).await;

        match result {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    async fn get(&self, key: &str) -> Result<String, RedisError> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let result: RedisResult<String> = con.get(key).await;

        match result {
            Ok(val) => Ok(val),
            Err(e) => Err(e),
        }
    }
}
