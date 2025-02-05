use anyhow::Result;
use serde::Deserialize;

use crate::web::service::{
    BadgeService, ImageProcBadgeService, K8DeploymentService, MiaDeploymentService,
};

#[derive(Clone, Debug, Deserialize)]
pub struct DeploymentState<
    MiaDeploymentSvc: MiaDeploymentService = K8DeploymentService,
    BadgeSvc: BadgeService = ImageProcBadgeService,
> {
    pub deployment_service: MiaDeploymentSvc,
    pub badge_service: BadgeSvc,
}

impl DeploymentState {
    pub fn new() -> Result<Self> {
        Ok(Self {
            deployment_service: Default::default(),
            badge_service: ImageProcBadgeService::new()?,
        })
    }
}
