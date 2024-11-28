use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "my_cli")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A CLI tool example with subcommands", long_about = None)]
struct Cli {
    /// Subcommand to execute
    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Event {
        #[command(subcommand)]
        action: EventActions,
    },
}

#[derive(Subcommand, Debug)]
enum EventActions {
    /// Publish an event
    Publish {
        /// The event source (e.g., "User.created")
        #[arg(short, long)]
        source: String,

        /// The event detail as JSON (e.g., "{}")
        #[arg(short, long)]
        detail: String,
    },

    /// Subscribe to an event
    Subscribe {
        /// The event source to subscribe to (e.g., "User.created")
        #[arg(short, long)]
        source: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.subcommand {
        Commands::Event { action } => match action {
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
