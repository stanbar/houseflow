use houseflow_token::Token;
use houseflow_types::{UserAgent, UserID};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub user_agent: UserAgent,
}

pub type LoginResponse = Result<LoginResponseBody, LoginError>;

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[serde(tag = "error", content = "error_description")]
pub enum LoginError {
    #[error("invalid password")]
    InvalidPassword,

    #[error("user not found")]
    UserNotFound,

    #[error("internal error: `{0}`")]
    // Replace it with better type if needed
    InternalError(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginResponseBody {
    pub refresh_token: Token,
    pub access_token: Token,
}

#[cfg(feature = "houseflow-db")]
impl From<houseflow_db::Error> for LoginError {
    fn from(v: houseflow_db::Error) -> Self {
        Self::InternalError(v.to_string())
    }
}


#[cfg(feature = "actix")]
impl actix_web::ResponseError for LoginError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InvalidPassword => StatusCode::BAD_REQUEST,
            Self::UserNotFound => StatusCode::NOT_FOUND,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let response = LoginResponse::Err(self.clone());
        let json = actix_web::web::Json(response);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}


//
// Register
// 

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

pub type RegisterResponse = Result<RegisterResponseBody, RegisterError>;

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
pub enum RegisterError {
    #[error("internal error: `{0}`")]
    // Replace it with better type if needed
    InternalError(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterResponseBody {
    pub user_id: UserID,
}

#[cfg(feature = "houseflow-db")]
impl From<houseflow_db::Error> for RegisterError {
    fn from(v: houseflow_db::Error) -> Self {
        Self::InternalError(v.to_string())
    }
}


#[cfg(feature = "actix")]
impl actix_web::ResponseError for RegisterError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let response = RegisterResponse::Err(self.clone());
        let json = actix_web::web::Json(response);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}
