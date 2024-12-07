use crate::domain::authdomain::{BaseResponse, Request, Token};
use crate::AppState;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::{Extension, Json};


// handler for generate token
pub async fn login(
    // header_map: HeaderMap,
    // Extension(s): Extension<String>,
    // State(state): Extension<AppState>,
    // Json(input): Json<Request>
) -> (StatusCode, BaseResponse<Token>) {
    //
    (StatusCode::OK, BaseResponse::new(Token{
        access_token: "".to_string(),
    }))
}