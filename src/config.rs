use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub config_path: String,
    pub config: Config,
}

impl AppContext {
    pub fn load(config_path: Option<&str>) -> Result<Self> {
        let config_path = config_path
            .map(|s| s.to_string())
            .unwrap_or_else(|| "guiding-light.toml".to_string());

        let config = if std::path::Path::new(&config_path).exists() {
            let raw = fs::read_to_string(&config_path)?;
            toml::from_str::<Config>(&raw)?
        } else {
            Config::default()
        };

        Ok(Self { config_path, config })
    }

    pub fn llm_enabled(&self) -> bool {
        let env = std::env::var("GL_LLM_ENABLED").ok();
        if let Some(v) = env {
            return matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES");
        }
        self.config.llm.enabled
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub journal: JournalConfig,
    pub llm: LlmConfig,

    #[serde(default)]
    pub policy: Vec<PolicyRuleConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            journal: JournalConfig {
                path: ".guiding-light/journal.jsonl".to_string(),
            },
            llm: LlmConfig {
                enabled: true,
                provider: "ollama".to_string(),
                base_url: "http://localhost:11434/api".to_string(),
                model: "gemma3".to_string(),
                timeout_ms: 120_000,
            },
            policy: vec![],
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JournalConfig {
    pub path: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LlmConfig {
    pub enabled: bool,
    pub provider: String,
    pub base_url: String,
    pub model: String,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PolicyRuleConfig {
    pub id: String,
    pub intent: Vec<String>,
    #[serde(rename = "type")]
    pub rule_type: String,
    pub pattern: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub message: String,
}

pub fn init_repo_files(force: bool) -> Result<()> {
    let cfg_path = std::path::Path::new("guiding-light.toml");
    let journal_dir = std::path::Path::new(".guiding-light");

    if !journal_dir.exists() {
        fs::create_dir_all(journal_dir)?;
    }

    if force || !cfg_path.exists() {
        let default_config = r#"[journal]
path = \".guiding-light/journal.jsonl\"

[llm]
enabled = true
provider = \"ollama\"
base_url = \"http://localhost:11434/api\"
model = \"gemma3\"
timeout_ms = 120000

[[policy]]
id = \"conventional-commits\"
intent = [\"commit_message\"]
type = \"regex_require\"
pattern = \"^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(\\\\([^)]+\\\\))?(!)?: .+\"
message = \"Commit message must follow Conventional Commits\"\n\n[[policy]]
id = \"no-exhausted-shipping\"
intent = [\"commit_message\", \"pre_push\", \"reflect\"]
type = \"keyword_block\"
keywords = [\"exhausted\", \"burnt out\", \"too tired\"]
message = \"If you're exhausted, slow down and rest.\"\n\"#;
        fs::write(cfg_path, default_config)?;
    }

    Ok(())
}
