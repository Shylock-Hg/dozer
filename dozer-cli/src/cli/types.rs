use clap::{Args, Parser, Subcommand};

use super::helper::{DESCRIPTION, LOGO};

use dozer_types::{
    constants::{DEFAULT_CONFIG_PATH_PATTERNS, LOCK_FILE},
    serde_json,
};

#[derive(Parser, Debug)]
#[command(author, version, name = "dozer")]
#[command(
    about = format!("{} \n {}", LOGO, DESCRIPTION),
    long_about = None,
)]
pub struct Cli {
    #[arg(
        global = true,
        short = 'c',
        long = "config-path",
        default_values = DEFAULT_CONFIG_PATH_PATTERNS
    )]
    pub config_paths: Vec<String>,
    #[arg(global = true, long, hide = true)]
    pub config_token: Option<String>,
    #[arg(global = true, long = "enable-progress")]
    pub enable_progress: bool,
    #[arg(global = true, long, value_parser(parse_config_override))]
    pub config_overrides: Vec<(String, serde_json::Value)>,

    #[arg(global = true, long = "ignore-pipe")]
    pub ignore_pipe: bool,
    #[clap(subcommand)]
    pub cmd: Commands,
}

fn parse_config_override(
    arg: &str,
) -> Result<(String, serde_json::Value), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut split = arg.split('=');
    let pointer = split.next().ok_or("missing json pointer")?;
    let value = split.next().ok_or("missing json value")?;
    Ok((pointer.to_string(), serde_json::from_str(value)?))
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(
        about = "Clean home directory",
        long_about = "Clean home directory. It removes all data, schemas and other files in app \
            directory"
    )]
    Clean,
    #[command(about = "Build YAML definitions as a dozer pipeline")]
    Build(Build),
    #[command(about = "Run a replication instance with the provided configuration")]
    Run,
    #[command(about = "Run UI server")]
    UI(UI),
}

#[derive(Debug, Args)]
pub struct UI {
    #[command(subcommand)]
    pub command: Option<UICommands>,
}

#[derive(Debug, Subcommand)]
pub enum UICommands {
    #[command(
        about = "Updates the latest UI code",
        long_about = "Updates the latest UI code"
    )]
    Update,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Live {
    #[arg(long, hide = true)]
    pub disable_live_ui: bool,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Build {
    #[arg(help = format!("Require that {LOCK_FILE} is up-to-date"), long = "locked")]
    pub locked: bool,
    #[arg(short = 'f')]
    pub force: Option<Option<String>>,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Deploy {
    pub target_url: String,
    #[arg(short = 'u')]
    pub username: Option<String>,
    #[arg(short = 'p')]
    pub password: Option<String>,
}

#[cfg(test)]
mod tests {
    use dozer_types::serde_json;

    #[test]
    fn test_parse_config_override_string() {
        let arg = "/app=\"abc\"";
        let result = super::parse_config_override(arg).unwrap();
        assert_eq!(result.0, "/app");
        assert_eq!(result.1, serde_json::Value::String("abc".to_string()));
    }

    #[test]
    fn test_parse_config_override_number() {
        let arg = "/app=123";
        let result = super::parse_config_override(arg).unwrap();
        assert_eq!(result.0, "/app");
        assert_eq!(result.1, serde_json::Value::Number(123.into()));
    }

    #[test]
    fn test_parse_config_override_object() {
        let arg = "/app={\"a\": 1}";
        let result = super::parse_config_override(arg).unwrap();
        assert_eq!(result.0, "/app");
        assert_eq!(
            result.1,
            serde_json::json!({
                "a": 1
            })
        );
    }
}
