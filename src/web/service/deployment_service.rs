use super::Service;

#[cfg_attr(test, mockall::automock)]
pub trait MiaDeploymentService: Service {
    fn get_version(&self, namespace: &str, service_name: &str) -> String;
}

#[derive(Clone, Default)]
pub struct K8DeploymentService();

impl MiaDeploymentService for K8DeploymentService {
    fn get_version(&self, namespace: &str, service_name: &str) -> String {
        let major = namespace.len();
        let minor = service_name.len();
        let patch = ((major + minor) % 2) as u8;
        format!("{major}.{minor}.{patch}")
    }
}
