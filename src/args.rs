use std::path::PathBuf;
use clap::{Subcommand, Args};

#[derive(Subcommand)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    pub path: Option<PathBuf>,
    pub chunk_type: Option<String>,
    pub message: Option<String>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    pub path: Option<PathBuf>,
    pub chunk_type: Option<String>,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    pub path: Option<PathBuf>,
    pub chunk_type: Option<String>,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    pub path: Option<PathBuf>,
}
