use anyhow::Result;
use mia_info_poc_macros::substate;

use crate::web::state::AppState;

use super::Service;

#[substate(AppState, field(deployment_service))]
#[cfg_attr(test, mockall::automock)]
pub trait MiaDeploymentService: Service {
    fn get_container_count(&self, namespace: &str, service_name: &str) -> Result<u32>;
    fn get_version(&self, namespace: &str, service_name: &str) -> String;
}

#[cfg(test)]
impl From<MockMiaDeploymentService>
    for axum::extract::State<std::sync::Arc<dyn MiaDeploymentService>>
{
    fn from(value: MockMiaDeploymentService) -> Self {
        axum::extract::State(std::sync::Arc::new(value))
    }
}

#[derive(Clone, Default)]
pub struct K8DeploymentService();

impl MiaDeploymentService for K8DeploymentService {
    fn get_container_count(&self, namespace: &str, service_name: &str) -> Result<u32> {
        Ok(((namespace.len() + service_name.len()) % 4) as u32)
    }

    fn get_version(&self, namespace: &str, service_name: &str) -> String {
        let major = namespace.len();
        let minor = service_name.len();
        let patch = ((major + minor) % 2) as u8;
        format!("{major}.{minor}.{patch}")
    }
}
