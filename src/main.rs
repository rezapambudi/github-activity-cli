mod cli;
use crate::cli::Cli;
use clap::Parser;
mod activities_printer;
mod github_client;

fn main() {
    Cli::parse().run_command();
}
