use clap::{
    Subcommand,
    Parser
};

#[derive(Debug, Parser)]
pub struct Cli{
    #[command(subcommand)]
    command: Commands
}


#[derive(Debug, Subcommand)]
enum Commands {
    compress,
    decompress
}
