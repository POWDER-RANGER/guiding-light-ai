mod cli;
mod config;
mod hooks;
mod journal;
mod llm;
mod policy;

use anyhow::Context as _;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let ctx = config::AppContext::load(cli.config.as_deref())?;

    match cli.command {
        cli::Command::Init { force } => {
            config::init_repo_files(force)?;
            println!("Initialized guiding-light files.");
            Ok(())
        }

        cli::Command::Reflect {
            text,
            intent,
            no_llm,
            journal,
        } => {
            let intent = intent.unwrap_or_else(|| "reflect".to_string());

            let report = policy::evaluate_text(&ctx.config, &intent, &text)?;
            report.print_human_advisory();

            let llm_enabled = ctx.llm_enabled() && !no_llm;
            let mut reflection: Option<String> = None;

            if llm_enabled {
                let prompt = llm::build_reflection_prompt(&ctx.config, &intent, &text, &report);
                reflection = Some(llm::ollama::generate(&ctx, &prompt)?);
            }

            if journal {
                journal::append_entry(
                    &ctx.config.journal.path,
                    journal::JournalEntry::new_reflection(&intent, &text, reflection.as_deref()),
                )?
            }

            if let Some(r) = reflection {
                println!("\\n---\\nLLM reflection:\\n{}\\n", r);
            }

            Ok(())
        }

        cli::Command::Policy(cli::PolicyCommand::Check { text, intent, fatal }) => {
            let intent = intent.unwrap_or_else(|| "policy_check".to_string());
            let report = policy::evaluate_text(&ctx.config, &intent, &text)?;
            report.print_human();

            if fatal && !report.passed() {
                std::process::exit(1);
            }
            if !report.passed() {
                std::process::exit(2);
            }
            Ok(())
        }

        cli::Command::Journal(cli::JournalCommand::Add { decision, why, tags }) => {
            journal::append_entry(
                &ctx.config.journal.path,
                journal::JournalEntry::new_decision(&decision, &why, tags),
            )?
            println!("Journaled decision.");
            Ok(())
        }

        cli::Command::Journal(cli::JournalCommand::List { limit, json }) => {
            let entries = journal::read_entries(&ctx.config.journal.path)
                .with_context(|| "Failed to read journal entries")?;
            let limit = limit.unwrap_or(50).min(entries.len());

            let slice = &entries[entries.len().saturating_sub(limit)..];

            if json {
                println!("{}", serde_json::to_string_pretty(slice)?);
            } else {
                for e in slice {
                    println!("{}", e.to_human());
                }
            }
            Ok(())
        }

        cli::Command::Hook(cli::HookCommand::Install { repo, hooks }) => {
            hooks::install_hooks(repo.as_deref(), hooks)?;
            Ok(())
        }

        cli::Command::Hook(cli::HookCommand::Run { hook_name, args }) => {
            hooks::run_hook(&ctx, &hook_name, &args)
        }
    }
}
