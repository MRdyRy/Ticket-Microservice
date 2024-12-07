use crate::handler::authhandler::login;
use crate::handler::healthcheck::healthcheck;
use crate::repository::http::{Http, UserRepository};
use crate::repository::redis::{RedisProvider, RedisRepository};
use crate::service::service::Service;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

mod domain {
    pub mod authdomain;
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
    pub mod authhandler;
    pub mod healthcheck;
}

#[derive(Clone, Debug)]
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
}

#[tokio::main]
async fn main() {
    let config = AppConfig {
        user_service: "localhost:8089".to_string(),
        redis_host: "127.0.0.1".to_string(),
        redis_port: 6379,
        redis_password: "".to_string(),
        app_addr: "localhost:8080".to_string(),
    };

    let redis = RedisRepository::new(&*config.redis_host, &*config.redis_password);
    let rest = Http::new(UserRepository {
        url: config.user_service.clone(),
    });

    let service = Service::new(Arc::new(redis), Arc::new(rest));
    let server_state = AppState { config, service };

    let app: Router = Router::new()
        .route("/", get(healthcheck))
        .route("/login", post(login))
        .with_state(server_state);

    let listen = tokio::net::TcpListener::bind(config.app_addr)
        .await
        .unwrap();
    axum::serve(listen, app).await;
}
