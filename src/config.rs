#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeploymentEnvironment {
    Testing,
    Production,
}

impl DeploymentEnvironment {
    pub fn from_env() -> Self {
        match std::env::var("DEPLOYMENT_ENVIRONMENT") {
            Ok(env) if env == "testing" => DeploymentEnvironment::Testing,
            _ => DeploymentEnvironment::Production,
        }
    }
}

pub struct Configuration {
    pub listen_addr: String,
    pub public_hostname: String,
}
