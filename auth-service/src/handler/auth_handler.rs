use crate::domain::auth_domain::{BaseResponse, Request, Token};
use crate::service::service::AuthService;
use crate::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Json as AxumJson},
};

// handler for validate user, generate token, and store to cache
#[utoipa::path(
post,
path="/auth",
request_body = Request,
responses(
        (status = 200, description = "Token generated successfully", body = BaseResponse<Token>),
        (status = 401, description = "Invalid credentials", body = BaseResponse<Token>),
        (status = 500, description = "Internal server error", body = BaseResponse<Token>)
    )
)]
pub async fn login_handler(
    State(app_state): State<AppState>,
    Json(req): Json<Request>,
) -> impl IntoResponse {
    tracing::info!("try authenticate for this req : {:?}", req);
    match app_state
        .service
        .generate_token(req, app_state.config.secret.clone())
        .await
    {
        Ok(Some(token)) => (
            StatusCode::OK,
            AxumJson(BaseResponse::new(Token {
                access_token: token,
            })),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::UNAUTHORIZED,
            AxumJson(BaseResponse::err(Token {
                access_token: "".to_string(),
            })),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            AxumJson(BaseResponse::err(Token {
                access_token: "".to_string(),
            })),
        )
            .into_response(),
    }
}
