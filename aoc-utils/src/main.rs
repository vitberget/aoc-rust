use clap::{Parser, Subcommand};

pub mod create;
pub mod download;

#[derive(Parser)]
pub struct AocArgs {
    #[command(subcommand)]
    cmd: Commands   
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Create { year: u16, day: u16 },
    Download { year: u16, day: u16 },
}

pub fn main() -> anyhow::Result<()> {
    let args = AocArgs::parse();

    match args.cmd {
        Commands::Create { year, day } => create::create(year, day),
        Commands::Download { year, day } => download::download(year, day),
    }
}

