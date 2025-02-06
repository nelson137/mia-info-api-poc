use axum::{
    extract::{Path, State},
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::web::{state::DeploymentState, tags};

pub fn routes() -> OpenApiRouter<DeploymentState> {
    OpenApiRouter::new()
        .routes(routes!(container_count_badge))
        .routes(routes!(version_badge))
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
            description = "A deployment version badge PNG.",
            body = PngResponse,
            content_type = mime::IMAGE_PNG.as_ref(),
            example = "<binary image data>",
        ),
        (status = INTERNAL_SERVER_ERROR, description = "Error."),
    ),
)]
pub async fn container_count_badge(
    State(state): State<DeploymentState>,
    Path((namespace, service_name)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    let count = state
        .deployment_service
        .get_container_count(&namespace, &service_name)
        .expect("get container count"); // TODO: handle error

    match state.badge_service.generate_count_badge(count) {
        Ok(image) => Ok(PngResponse(image)),
        Err(err) => {
            // eprintln!("Error generating badge for version {version}");
            eprintln!("{err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
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
    State(state): State<DeploymentState>,
    Path((namespace, service_name)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    let version = state
        .deployment_service
        .get_version(&namespace, &service_name);

    match state.badge_service.generate_version_badge(&version) {
        Ok(image) => Ok(PngResponse(image)),
        Err(err) => {
            eprintln!("Error generating badge for version {version}");
            eprintln!("{err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
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
        web::{
            service::{BadgeService, MockBadgeService, MockMiaDeploymentService},
            state::DeploymentState,
        },
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
                .with(pred::eq(version))
                .return_once(move |_| Ok(gen_badge_bytes));
            Ok(svc)
        });
        let badge_svc = MockBadgeService::new().unwrap();

        let state = DeploymentState::from_parts(deployment_svc, badge_svc);
        let actual_badge = version_badge(State(state), Path((namespace, service_name)))
            .await
            .read_response_as_bytes()
            .await;

        assert_eq!(expected_badge, actual_badge);
    }
}
