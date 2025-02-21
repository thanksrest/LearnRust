mod cli;
mod web;
mod lessons;
mod exercises;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Interactive,
    Web,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Interactive) => {
            let mut interactive_cli = cli::interactive_lesson::InteractiveLessonCLI::new();
            interactive_cli.run().await?;
        }
        Some(Commands::Web) => {
            web::server::run_server().await?;
        }
        None => {
            println!("Please specify a command: interactive or web");
        }
    }

    Ok(())
}
