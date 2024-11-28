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
