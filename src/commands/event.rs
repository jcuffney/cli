use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum EventActions {
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

#[derive(Args, Debug)]
pub struct EventCommand {
    #[command(subcommand)]
    pub action: EventActions,
}

pub fn publish(source: &str, detail: &str) {
    println!("Publishing event:");
    println!("Source: {}", source);
    println!("Detail: {}", detail);
}

pub fn subscribe(source: &str) {
    println!("Subscribing to event:");
    println!("Source: {}", source);
}
