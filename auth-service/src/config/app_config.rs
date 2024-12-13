use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub user_service: String,
    pub redis_host: String,
    pub redis_port: u16,
    pub redis_password: String,
    pub app_addr: String,
    pub secret: String,
}

impl AppConfig {
    pub fn load() -> AppConfig {
        dotenv::dotenv().ok();
        AppConfig {
            user_service: env::var("user.service.url").expect("Error user service param not set!"),
            redis_host: env::var("redis.host").expect("Error redis.host param not set!"),
            redis_port: env::var("redis.port")
                .unwrap_or_else(|_| "6379".to_string())
                .parse()
                .expect("Error redis.port param not set"),
            redis_password: env::var("redis.password")
                .expect("Error redis.password param not set!"),
            app_addr: env::var("app.service.url").expect("Error app service param not set!"),
            secret: env::var("secret").expect("Error secret param not set!"),
        }
    }
}
