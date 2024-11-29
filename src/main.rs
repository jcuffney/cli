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

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let config = aws_config::load_from_env().await;
    let cw_client = aws_sdk_cloudwatchevents::Client::new(&config);

    match args.subcommand {
        Commands::Event(event) => match event.action {
            EventActions::Publish { source, detail } => {
                commands::event::publish(cw_client, &source, &detail).await;
            }
            EventActions::Subscribe { source } => {
                commands::event::subscribe(cw_client, &source).await;
            }
        },
    }
}
