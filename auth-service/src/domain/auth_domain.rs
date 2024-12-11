use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Default, Clone, ToSchema)]
pub struct BaseResponse<T> {
    #[schema(example = "OK")]
    status: String,
    data: Option<T>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    #[schema(example = "Bearer <JWT token>")]
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct Request {
    #[schema(example = "rudy.ryanto@example.com")]
    pub email: String,
    #[schema(example = "password123")]
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct UserResponse {
    #[schema(example = true)]
    pub valid: bool,
}

impl<T : std::default::Default > BaseResponse<T> {
    pub fn new (data: T) -> BaseResponse<T>{
        BaseResponse::<T> {
            status: "OK".to_string(),
            data: Some(data),
        }
    }

    pub fn err (data: T) -> BaseResponse<T> {
        BaseResponse::<T> {
            status: "Error".to_string(),
            data: Some(data),
        }
    }
}