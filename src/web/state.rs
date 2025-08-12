use std::sync::Arc;

use crate::{error::Result, web::service::*};

pub type OpenApiRouter = utoipa_axum::router::OpenApiRouter<AppState>;

#[derive(Clone)]
pub struct AppState {
    pub deployment_service: Arc<dyn MiaDeploymentService>,
    pub badge_service: Arc<dyn BadgeService>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        Ok(Self {
            deployment_service: Arc::new(K8DeploymentService::default()),
            badge_service: Arc::new(ImageProcBadgeService::new()?),
        })
    }
}
