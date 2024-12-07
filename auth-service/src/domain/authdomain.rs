use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct BaseResponse<T> {
    status: String,
    data: Option<T>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub email: String,
    pub password: String,
}
#[derive(Serialize, Deserialize)]
pub struct UserResponse {
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