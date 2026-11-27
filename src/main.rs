#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "betteroffice-docs", about = "SigmaOS BetterOffice document editor")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("betteroffice-docs: no entries configured."),
        Commands::Status => println!("betteroffice-docs: ready."),
    }
}
