use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    /// The input path of the archive.dht file.
    pub input: PathBuf,
    /// The output path of the dht.json file.
    pub output: PathBuf,
}
