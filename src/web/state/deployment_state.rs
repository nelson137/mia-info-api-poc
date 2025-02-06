use std::sync::Arc;

use anyhow::Result;

use crate::web::service::{
    BadgeService, ImageProcBadgeService, K8DeploymentService, MiaDeploymentService,
};

#[derive(Clone)]
pub struct DeploymentState {
    pub deployment_service: Arc<dyn MiaDeploymentService>,
    pub badge_service: Arc<dyn BadgeService>,
}

impl DeploymentState {
    pub fn new() -> Result<Self> {
        Ok(Self {
            deployment_service: Arc::new(K8DeploymentService::default()),
            badge_service: Arc::new(ImageProcBadgeService::new()?),
        })
    }

    #[cfg(test)]
    pub fn from_parts<DeploymentSvc: MiaDeploymentService, BadgeSvc: BadgeService>(
        deployment_svc: DeploymentSvc,
        badge_svc: BadgeSvc,
    ) -> DeploymentState {
        DeploymentState {
            deployment_service: Arc::new(deployment_svc),
            badge_service: Arc::new(badge_svc),
        }
    }
}
