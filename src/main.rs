mod cli;
mod convert_db_to_legacy;
mod parse;

use crate::cli::Cli;
use crate::convert_db_to_legacy::convert_database_to_dht;
use clap::Parser;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let dht = convert_database_to_dht(&cli.input).await?;
    let output_json = serde_json::to_string(&dht)?;
    File::create(&cli.output)?.write_all(output_json.as_bytes())?;
    println!("File written to {:?}", cli.output);
    Ok(())
}
