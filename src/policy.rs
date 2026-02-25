use anyhow::Result;
use serde::Deserialize;
use colored::Colorize;

#[derive(Debug, Deserialize)]
pub struct Policy {
    pub version: u32,
    pub name: String,
    #[serde(default)]
    pub principles: Vec<String>,
    #[serde(default)]
    pub banned_phrases: Vec<String>,
    #[serde(default)]
    pub required_when_risky: RequiredWhenRisky,
}

#[derive(Debug, Deserialize, Default)]
pub struct RequiredWhenRisky {
    #[serde(default)]
    pub triggers: Vec<String>,
    #[serde(default)]
    pub require_intent_statement: bool,
}

#[derive(Debug)]
pub struct CheckResult {
    pub ok: bool,
    pub summary: String,
    pub violations: Vec<String>,
    pub risky: bool,
}

impl CheckResult {
    pub fn print_human(&self) {
        if self.ok {
            println!("{}", "✓ Policy check passed".green());
        } else {
            println!("{}", "✗ Policy violations".red());
            for v in &self.violations {
                println!("  {}", v.yellow());
            }
        }
    }
}

impl Policy {
    pub fn load(path: &std::path::Path) -> Result<Self> {
        let bytes = std::fs::read(path)?;
        Ok(serde_yaml::from_slice(&bytes)?)
    }
}

pub fn extract_added_lines(diff: &str) -> String {
    diff.lines()
        .filter_map(|line| {
            if line.starts_with("+++") {
                return None;
            }
            if line.starts_with('+') {
                Some(line.trim_start_matches('+'))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn extract_intent_from_text(text: &str) -> Option<String> {
    for line in text.lines() {
        let l = line.trim();
        let lower = l.to_lowercase();
        if lower.starts_with("intent:") || lower.starts_with("gl-intent:") {
            return Some(l.splitn(2, ':').nth(1)?.trim().to_string());
        }
    }
    None
}

pub fn check_text(policy: &Policy, text: &str, intent: Option<&str>) -> CheckResult {
    let t = text.to_lowercase();
    let mut violations = vec![];

    for banned in &policy.banned_phrases {
        let b = banned.to_lowercase();
        if !b.is_empty() && t.contains(&b) {
            violations.push(format!("Banned phrase: '{}'", banned));
        }
    }

    let mut risky = false;
    for trig in &policy.required_when_risky.triggers {
        let tr = trig.to_lowercase();
        if !tr.is_empty() && t.contains(&tr) {
            risky = true;
            break;
        }
    }

    if risky && policy.required_when_risky.require_intent_statement {
        if intent.map(|s| s.trim().is_empty()).unwrap_or(true) {
            violations.push("Risk detected: intent statement required".into());
        }
    }

    CheckResult {
        ok: violations.is_empty(),
        summary: if violations.is_empty() {
            format!("Aligned with '{}'", policy.name)
        } else {
            format!("Violations in '{}'", policy.name)
        },
        violations,
        risky,
    }
}
