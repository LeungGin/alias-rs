use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
    #[arg(short = 'd', long = "define")]
    pub runtime_variables: Vec<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Set alias
    Set {
        /// Alias what you want
        #[arg(index = 1)]
        alias: String,
        /// Command executed by alias
        #[arg(index = 2)]
        command: String,
        /// Command executed by alias
        #[arg(short = 'g', long = "group")]
        group: Option<String>,
    },
    /// Remove alias
    Remove {
        /// Alias which you want to remove
        alias: String,
    },
    /// List all aliases
    List {},
    /// Clear all aliases
    Clear {},
    /// Export all aliases config
    Export {
        /// Export file path (include file name)
        #[arg(index = 1)]
        export_path: String,
    },
    /// Import aliases config
    Import {
        /// Import file path (include file name)
        #[arg(index = 1)]
        import_path: String,
    },
    /// Fix alias config when the alias config fails
    Rebuild {
        /// Specified setting file path
        #[arg(index = 1)]
        setting_path: String,
    },
}
