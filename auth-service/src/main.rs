use crate::handler::auth_handler::login_handler;
use crate::handler::healthcheck::healthcheck;
use crate::repository::http::{RestRepository, UserRepository};
use crate::repository::redis::RedisRepository;
use crate::service::service::Service;
use axum::routing::{get, post, Router};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::config::app_config::AppConfig;
use crate::handler::auth_handler::__path_login_handler;
use crate::domain::auth_domain::{Request,Token, BaseResponse};

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

mod config {
    pub mod app_config;
}
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub service: Service,
}



#[derive(OpenApi)]
#[openapi(
    info(
        title = "Authentication-Api",
        description = "API documentation for Authentication",
        version = "1.0.0",
        contact(name = "Rudy Ryanto", email = "ryanserfanru@gmail.com"),
    ),
    paths(
        login_handler,
    ),
    components(
        schemas(Request, Token, BaseResponse<Token>)
    ),
    tags(
        (name = "auth", description = "Authentication API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = AppConfig::load();

    let redis = RedisRepository::new(&config.redis_host, &config.redis_password);
    let rest = RestRepository::new(UserRepository {
        url: config.user_service.clone(),
    });

    let service = Service::new(Arc::new(redis), Arc::new(rest));
    let app_state = AppState { config, service };

    let app = Router::new()
        .route("/", get(healthcheck))
        .route("/auth", post(login_handler))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(app_state.clone());

    let listen = tokio::net::TcpListener::bind(app_state.clone().config.app_addr)
        .await
        .unwrap();
    axum::serve(listen, app).await.expect("Failed to listen !!");
    tracing::info!("app running on port : {}", app_state.config.app_addr)
}
