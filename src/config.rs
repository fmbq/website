use serde::Deserialize;

/// Global configuration for running the website. These are normally pulled from
/// environment variables, or from a `.env` file.
#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    #[serde(default)]
    pub deployment_environment: DeploymentEnvironment,

    #[serde(default)]
    pub dynamic_image_conversions: bool,

    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,

    #[serde(default = "default_database_url")]
    pub database_url: String,

    pub redis_host: Option<String>,
    pub redis_port: Option<u16>,
}

impl Configuration {
    pub fn from_env() -> envy::Result<Self> {
        envy::from_env()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeploymentEnvironment {
    #[default]
    Testing,
    Production,
}

fn default_listen_addr() -> String {
    "localhost:5000".into()
}

fn default_database_url() -> String {
    "sqlite://sqlite.db".into()
}
