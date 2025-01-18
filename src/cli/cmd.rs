use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    /// sub command
    #[command(subcommand)]
    pub command: Command,
    /// alias setting file path
    #[arg(long = "setting")]
    pub setting_path: Option<String>,
    /// runtime variable define, allow multiple inputs.
    /// e.g. --define var_1=xxx --define var_2=xxx
    #[arg(long = "define")]
    pub runtime_variables: Vec<String>,
}

#[derive(Subcommand)]
pub enum Command {
    /// initialize system env setting
    Init {},
    /// set alias
    Set {
        /// alias what you want
        #[arg(index = 1)]
        alias: String,
        /// alias mapping command
        #[arg(index = 2)]
        command: String,
    },
    /// remove alias
    Remove {
        /// alias which you want to remove
        alias: String,
    },
    /// list aliases
    List {},
    /// export aliases define
    Export {
        /// export path (include file name)
        export_path: String,
    },
    /// import aliases define
    Import {},
}
