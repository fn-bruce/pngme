use args::PngMeArgs;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod download;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: PngMeArgs,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    
    match args.command {
        PngMeArgs::Encode(args) => commands::encode(args),
        PngMeArgs::Decode(args) => commands::decode(args),
        PngMeArgs::Remove(args) => commands::remove(args),
        PngMeArgs::Print(args) => commands::print_chunks(args),
    }?;

    Ok(())
}
