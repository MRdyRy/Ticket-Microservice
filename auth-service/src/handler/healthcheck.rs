//for checking liveness and readiness apss
pub async fn healthcheck() -> &'static str {
    "Pong! apps is running healthy"
}