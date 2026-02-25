pub mod ollama;

use anyhow::Result;
use crate::config::Config;

pub async fn reflect(cfg: &Config, text: &str) -> Result<String> {
    if let (Some(base), Some(model)) = (&cfg.llm.base_url.clone(), &cfg.llm.model.clone()) {
        if cfg.llm.enabled {
            return ollama::reflect(base, model, text).await;
        }
    }

    Ok(format!(
        r#"GUIDING LIGHT (offline mode)

You shared:
  "{}"

Consider:
1. What's the real need here?
2. What's the smallest reversible action?
3. Will you respect yourself tomorrow?"#,
        text
    ))
}
