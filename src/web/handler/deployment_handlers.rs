use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use crate::web::{
    service::{BadgeService, MiaDeploymentService},
    state::DeploymentState,
};

pub async fn badge<D: MiaDeploymentService, B: BadgeService>(
    State(state): State<DeploymentState<D, B>>,
    Path((namespace, service_name)): Path<(String, String)>,
) -> impl IntoResponse {
    let version = state
        .deployment_service
        .get_version(&namespace, &service_name);

    state.badge_service.generate_badge(&version)
}

#[cfg(test)]
mod tests {
    use mockall::predicate as pred;

    use crate::{
        test_utils::{self as utils, ReadResponseBody},
        web::service::{MockBadgeService, MockMiaDeploymentService},
    };

    use super::*;

    #[tokio::test]
    async fn test_badge() {
        let namespace = utils::rand_string();
        let service_name = utils::rand_string();
        let version = "7.16.1";

        let mut deployment_service = MockMiaDeploymentService::new();
        deployment_service
            .expect_get_version()
            .with(pred::always(), pred::always())
            .return_const(version);

        let gen_badge_bytes = utils::rand_vec_u8();
        let expected_badge = gen_badge_bytes.clone();
        let badge_service_ctx = MockBadgeService::new_context();
        badge_service_ctx.expect().returning(move || {
            let mut svc = MockBadgeService::default();
            svc.expect_generate_badge()
                .with(pred::eq(version))
                .return_const(gen_badge_bytes.clone());
            Ok(svc)
        });
        let badge_service = MockBadgeService::new().unwrap();

        let state = DeploymentState {
            deployment_service,
            badge_service,
        };
        let actual_badge = badge(State(state), Path((namespace, service_name)))
            .await
            .read_response_as_bytes()
            .await;

        assert_eq!(expected_badge, actual_badge);
    }
}
