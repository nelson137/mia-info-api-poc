use axum::response::IntoResponse;
use thiserror::Error;

use crate::web::models::JsonResponse;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("failed to parse font: {0}")]
    InvalidFont(String),

    #[error("failed to generate badge")]
    GenerateBadge {
        #[from]
        source: imageproc::image::ImageError,
    },
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::InvalidFont(..) => server_error_response(),
            Self::GenerateBadge { .. } => json_response(self.to_string()),
        }
    }
}

fn server_error_response() -> axum::response::Response {
    axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

fn json_response(message: String) -> axum::response::Response {
    JsonResponse::error(message).into_response()
}
