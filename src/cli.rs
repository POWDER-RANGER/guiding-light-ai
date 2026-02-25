use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "gl")]
#[command(about = "Guiding Light AI — values -> policies -> enforceable gates", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Path to guiding-light config TOML (default: ./guiding-light.toml)
    #[arg(long)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create default config/journal scaffolding
    Init {
        /// Overwrite existing files
        #[arg(long)]
        force: bool,
    },

    /// Reflect on a situation
    Reflect {
        text: String,

        /// Intent label for policies
        #[arg(long)]
        intent: Option<String>,

        /// Disable LLM
        #[arg(long)]
        no_llm: bool,

        /// Append to journal
        #[arg(long, default_value_t = true)]
        journal: bool,
    },

    /// Policy engine
    #[command(subcommand)]
    Policy(PolicyCommand),

    /// Decision journal
    #[command(subcommand)]
    Journal(JournalCommand),

    /// Git hook management
    #[command(subcommand)]
    Hook(HookCommand),
}

#[derive(Subcommand, Debug)]
pub enum PolicyCommand {
    /// Check text against policies
    Check {
        #[arg(long)]
        text: String,

        #[arg(long)]
        intent: Option<String>,

        /// Exit 1 on policy fail
        #[arg(long)]
        fatal: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum JournalCommand {
    Add {
        decision: String,

        #[arg(long)]
        why: String,

        /// Comma-separated tags
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,
    },
    List {
        #[arg(long)]
        limit: Option<usize>,

        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum HookCommand {
    /// Install Git hooks
    Install {
        /// Repo path
        #[arg(long)]
        repo: Option<String>,

        /// Hooks to install
        #[arg(long, value_delimiter = ',', default_value = "commit-msg,pre-push")]
        hooks: Vec<String>,
    },

    /// Internal: run hook
    Run {
        hook_name: String,

        /// Hook args from Git
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
}
