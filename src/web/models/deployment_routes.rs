use axum::response::{IntoResponse, Response};

use super::JsonResponse;

//=============================================================================
// Request
//=============================================================================

/// Query parameters for the [`container_count_badge`] endpoint.
#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct ContainersBadgeQuery {
    pub bg: Option<String>,
    pub fg: Option<String>,
}

/// Query parameters for the [`version_badge`] endpoint.
#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct VersionBadgeQuery {
    pub bg: Option<String>,
    pub fg: Option<String>,
}

//=============================================================================
// Response
//=============================================================================

#[derive(utoipa::ToSchema, serde::Serialize)]
pub struct ContainerCountResponse {
    pub namespace: String,
    pub service: String,
    pub containers: u32,
}

impl IntoResponse for ContainerCountResponse {
    fn into_response(self) -> Response {
        JsonResponse::new(vec![self]).into_response()
    }
}

#[derive(utoipa::ToSchema, serde::Serialize)]
pub struct DeploymentVersionResponse {
    pub namespace: String,
    pub service: String,
    pub version: String,
}

impl IntoResponse for DeploymentVersionResponse {
    fn into_response(self) -> Response {
        JsonResponse::new(vec![self]).into_response()
    }
}
