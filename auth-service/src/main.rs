use crate::handler::auth_handler::{login_handler};
use crate::handler::healthcheck::healthcheck;
use crate::repository::http::{RestRepository, UserRepository};
use crate::repository::redis::{RedisRepository};
use crate::service::service::Service;
use axum::routing::{get, post, Router};
use std::sync::Arc;

mod domain {
    pub mod auth_domain;
    pub mod claims;
}
mod repository {
    pub mod http;
    pub mod redis;
}
mod service {
    pub mod authentication;
    pub mod service;
}
mod handler {
    pub mod auth_handler;
    pub mod healthcheck;
}

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub service: Service,
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub user_service: String,
    pub redis_host: String,
    pub redis_port: u16,
    pub redis_password: String,
    pub app_addr: String,
    pub secret: String,
}

#[tokio::main]
async fn main() {
    let config = AppConfig {
        user_service: "localhost:8089".to_string(),
        redis_host: "127.0.0.1".to_string(),
        redis_port: 6379,
        redis_password: "".to_string(),
        app_addr: "localhost:8080".to_string(),
        secret: "".to_string(),
    };

    let redis = RedisRepository::new(&*config.redis_host, &*config.redis_password);
    let rest = RestRepository::new(UserRepository {
        url: config.user_service.clone(),
    });

    let service = Service::new(Arc::new(redis), Arc::new(rest));
    let app_state = AppState { config, service };

    let app = Router::new()
        .route("/", get(healthcheck))
        .route("/auth", post(login_handler))
        .with_state(app_state);

    let listen = tokio::net::TcpListener::bind(config.app_addr)
        .await
        .unwrap();
    axum::serve(listen, app).await.expect("Failed to listen !!");
}
