use clap::{Parser, Subcommand};

mod commands;
use commands::{event::EventActions, event::EventCommand};

#[derive(Parser, Debug)]
#[command(name = "cuffney", about = "cuffney platform cli tool")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Event(EventCommand),
}

fn main() {
    let args = Cli::parse();

    match args.subcommand {
        Commands::Event(event) => match event.action {
            EventActions::Publish { source, detail } => {
                println!("Publishing event:");
                println!("Source: {}", source);
                println!("Detail: {}", detail);
            }
            EventActions::Subscribe { source } => {
                println!("Subscribing to event:");
                println!("Source: {}", source);
            }
        },
    }
}
