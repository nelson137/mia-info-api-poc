mod badge_service;
pub use badge_service::*;

mod deployment_service;
pub use deployment_service::*;

pub trait Service: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Service for T {}
