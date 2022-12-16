use std::fs;

use crate::errors::AdminError;
use dozer_types::constants::DEFAULT_HOME_DIR;
use dozer_types::models::api_config::{default_api_config, ApiInternal};
use dozer_types::serde::{Deserialize, Serialize};
use dozer_types::serde_yaml;
pub mod cli_process;
pub mod types;
pub mod utils;
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct AdminCliConfig {
    pub port: u32,
    pub host: String,
    pub cors: bool,
    #[serde(default = "default_api_internal")]
    pub api_internal: ApiInternal,
    #[serde(default = "default_pipeline_internal")]
    pub pipeline_internal: ApiInternal,
    pub dozer_config: Option<String>,
    #[serde(default = "default_home_dir")]
    pub home_dir: String,
}
fn default_home_dir() -> String {
    DEFAULT_HOME_DIR.to_owned()
}
fn default_api_internal() -> ApiInternal {
    AdminCliConfig::default().api_internal
}
fn default_pipeline_internal() -> ApiInternal {
    AdminCliConfig::default().pipeline_internal
}
impl Default for AdminCliConfig {
    fn default() -> Self {
        let default_config = default_api_config();
        Self {
            port: 8081,
            host: "[::0]".to_owned(),
            cors: true,
            dozer_config: None,
            home_dir: default_home_dir(),
            api_internal: default_config.api_internal.unwrap(),
            pipeline_internal: default_config.pipeline_internal.unwrap(),
        }
    }
}
pub fn load_config(config_path: String) -> Result<AdminCliConfig, AdminError> {
    let contents = fs::read_to_string(config_path).map_err(AdminError::FailedToLoadFile)?;
    let config: AdminCliConfig =
        serde_yaml::from_str(&contents).map_err(|e| AdminError::FailedToParseYaml(Box::new(e)))?;
    Ok(config)
}