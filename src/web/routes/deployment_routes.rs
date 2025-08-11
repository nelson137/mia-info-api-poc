use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::{HeaderValue, header},
    response::{IntoResponse, Response},
};
use utoipa_axum::routes;

use crate::{
    error::Result,
    utils::parse_hex_string,
    web::{
        models::JsonResponse,
        service::{BadgeService, MiaDeploymentService},
        state::OpenApiRouter,
        tags,
    },
};

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(container_count))
        .routes(routes!(container_count_badge))
        .routes(routes!(version))
        .routes(routes!(version_badge))
}

#[utoipa::path(
    get,
    path = "/{namespace}/{service}/containers",
    tag = tags::MIA_DEPLOYMENT,
    summary = "get the container count of a deployment",
    params(
        ("namespace", Path, description = "The cluster namespace", example = "vcce-dev"),
        ("service", Path, description = "The service name", example = "memo-api"),
    ),
    responses(
        (
            status = OK,
            description = "Container count information.",
            body = ContainerCountResponse,
            content_type = mime::JSON.as_ref(),
            example = r#"{ "namespace": "vcce-dev", "service": "memo-api", "containers": 3 }"#,
        ),
        (status = INTERNAL_SERVER_ERROR, description = "Error."),
    ),
)]
pub async fn container_count(
    State(deployment_service): State<Arc<dyn MiaDeploymentService>>,
    Path((namespace, service_name)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    let count = deployment_service.get_container_count(&namespace, &service_name)?;
    Ok(ContainerCountResponse {
        namespace,
        service: service_name,
        containers: count,
    })
}

#[derive(utoipa::ToSchema, serde::Serialize)]
struct ContainerCountResponse {
    namespace: String,
    service: String,
    containers: u32,
}

impl IntoResponse for ContainerCountResponse {
    fn into_response(self) -> Response {
        JsonResponse::new(vec![self]).into_response()
    }
}

/// Query parameters for the [`container_count_badge`] endpoint.
#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct ContainersBadgeQuery {
    bg: Option<String>,
    fg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/{namespace}/{service}/containers/badge",
    tag = tags::MIA_DEPLOYMENT,
    summary = "generate deployment container count badge",
    params(
        ("namespace", Path, description = "The cluster namespace", example = "vcce-dev"),
        ("service", Path, description = "The service name", example = "memo-api"),
    ),
    responses(
        (
            status = OK,
            description = "A deployment container count badge PNG.",
            body = PngResponse,
            content_type = mime::IMAGE_PNG.as_ref(),
            example = "<binary image data>",
        ),
        (status = INTERNAL_SERVER_ERROR, description = "Error."),
    ),
)]
pub async fn container_count_badge(
    State(deployment_service): State<Arc<dyn MiaDeploymentService>>,
    State(badge_service): State<Arc<dyn BadgeService>>,
    Path((namespace, service_name)): Path<(String, String)>,
    Query(query): Query<ContainersBadgeQuery>,
) -> Result<impl IntoResponse> {
    let bg = query.bg.as_deref().map(parse_hex_string).transpose()?;
    let fg = query.fg.as_deref().map(parse_hex_string).transpose()?;

    let count = deployment_service.get_container_count(&namespace, &service_name)?;
    let image = badge_service.generate_count_badge(count, bg, fg)?;

    Ok(PngResponse(image))
}

#[utoipa::path(
    get,
    path = "/{namespace}/{service}/version",
    tag = tags::MIA_DEPLOYMENT,
    summary = "get the deployment version",
    params(
        ("namespace", Path, description = "The cluster namespace", example = "vcce-dev"),
        ("service", Path, description = "The service name", example = "memo-api"),
    ),
    responses(
        (
            status = OK,
            description = "Deployment version information.",
            body = DeploymentVersionResponse,
            content_type = mime::JSON.as_ref(),
            example = r#"{ "namespace": "vcce-dev", "service": "memo-api", "version": "1.2.3" }"#,
        ),
        (status = INTERNAL_SERVER_ERROR, description = "Error."),
    ),
)]
pub async fn version(
    State(deployment_service): State<Arc<dyn MiaDeploymentService>>,
    Path((namespace, service_name)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    let version = deployment_service.get_version(&namespace, &service_name);
    Ok(DeploymentVersionResponse {
        namespace,
        service: service_name,
        version,
    })
}

#[derive(utoipa::ToSchema, serde::Serialize)]
struct DeploymentVersionResponse {
    namespace: String,
    service: String,
    version: String,
}

impl IntoResponse for DeploymentVersionResponse {
    fn into_response(self) -> Response {
        JsonResponse::new(vec![self]).into_response()
    }
}

/// Query parameters for the [`version_badge`] endpoint.
#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct VersionBadgeQuery {
    bg: Option<String>,
    fg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/{namespace}/{service}/version/badge",
    tag = tags::MIA_DEPLOYMENT,
    summary = "generate deployment version badge",
    params(
        ("namespace", Path, description = "The cluster namespace", example = "vcce-dev"),
        ("service", Path, description = "The service name", example = "memo-api"),
    ),
    responses(
        (
            status = OK,
            description = "A deployment version badge PNG.",
            body = PngResponse,
            content_type = mime::IMAGE_PNG.as_ref(),
            example = "<binary image data>",
        ),
        (status = INTERNAL_SERVER_ERROR, description = "Error."),
    ),
)]
pub async fn version_badge(
    State(deployment_service): State<Arc<dyn MiaDeploymentService>>,
    State(badge_service): State<Arc<dyn BadgeService>>,
    Path((namespace, service_name)): Path<(String, String)>,
    Query(query): Query<VersionBadgeQuery>,
) -> Result<impl IntoResponse> {
    let bg = query.bg.as_deref().map(parse_hex_string).transpose()?;
    let fg = query.fg.as_deref().map(parse_hex_string).transpose()?;

    let version = deployment_service.get_version(&namespace, &service_name);
    let image = badge_service.generate_version_badge(&version, bg, fg)?;

    Ok(PngResponse(image))
}

#[derive(utoipa::ToSchema)]
struct PngResponse(Vec<u8>);

impl IntoResponse for PngResponse {
    fn into_response(self) -> Response {
        let content_type = (
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::IMAGE_PNG.as_ref()),
        );
        ([content_type], self.0).into_response()
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate as pred;

    use crate::{
        test_utils::*,
        web::service::{BadgeService, MockBadgeService, MockMiaDeploymentService},
    };

    use super::*;

    #[tokio::test]
    async fn test_version_badge() {
        let namespace = rand_string();
        let service_name = rand_string();
        let version = "7.16.1";

        let mut deployment_svc = MockMiaDeploymentService::new();
        deployment_svc
            .expect_get_version()
            .with(pred::always(), pred::always())
            .return_const(version);

        let gen_badge_bytes = rand_vec_u8();
        let expected_badge = gen_badge_bytes.clone();
        let badge_service_ctx = MockBadgeService::new_context();
        badge_service_ctx.expect().returning(move || {
            let mut svc = MockBadgeService::default();
            let gen_badge_bytes = gen_badge_bytes.clone();
            svc.expect_generate_version_badge()
                .with(pred::eq(version), pred::always(), pred::always())
                .return_once(move |_, _, _| Ok(gen_badge_bytes));
            Ok(svc)
        });
        let badge_svc = MockBadgeService::new().unwrap();

        let query = VersionBadgeQuery::default();

        let actual_badge = version_badge(
            deployment_svc.into(),
            badge_svc.into(),
            Path((namespace, service_name)),
            Query(query),
        )
        .await
        .read_response_as_bytes()
        .await;

        assert_eq!(expected_badge, actual_badge);
    }
}
