use crate::Commands;
pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "pasops")]
#[command(about = "Run create badges and git commands", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
