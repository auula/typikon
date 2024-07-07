use clap::Parser;
use typikon::cli::Cli;

fn main() {
    let cli = Cli::parse();

    cli.run();
}
