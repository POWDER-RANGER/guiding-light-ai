use anyhow::{Context, Result};
use clap::ValueEnum;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum HookMode {
    Strict,
    Warn,
}

pub fn install_hooks(mode: HookMode) -> Result<()> {
    let hooks_dir = ".git/hooks";
    fs::create_dir_all(hooks_dir).context("failed to create hooks dir")?;

    let mode_str = match mode {
        HookMode::Strict => "strict",
        HookMode::Warn => "warn",
    };

    let commit_msg_hook = format!(r#"#!/usr/bin/env bash
set -euo pipefail
MSG_FILE="${{1:-}}"
[[ -z "$MSG_FILE" || ! -f "$MSG_FILE" ]] && exit 0
GL_BIN="$(command -v gl || true)"
[[ -z "$GL_BIN" ]] && exit 0
POLICY_PATH="${{GL_POLICY_PATH:-policies/default.yml}}"
"$GL_BIN" policy check --file "$MSG_FILE" --diff --policy "$POLICY_PATH" --mode {mode_str}
"#);

    let pre_commit_hook = format!(r#"#!/usr/bin/env bash
set -euo pipefail
GL_BIN="$(command -v gl || true)"
[[ -z "$GL_BIN" ]] && exit 0
POLICY_PATH="${{GL_POLICY_PATH:-policies/default.yml}}"
"$GL_BIN" policy check --diff --policy "$POLICY_PATH" --mode {mode_str}
"#);

    write_hook(Path::new(hooks_dir).join("commit-msg"), &commit_msg_hook)?;
    write_hook(Path::new(hooks_dir).join("pre-commit"), &pre_commit_hook)?;

    Ok(())
}

fn write_hook(path: impl AsRef<Path>, content: &str) -> Result<()> {
    fs::write(&path, content)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms)?;
    }
    Ok(())
}
