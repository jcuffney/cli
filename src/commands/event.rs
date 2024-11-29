use aws_sdk_cloudwatchevents::types::PutEventsRequestEntry;
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

pub async fn publish(client: aws_sdk_cloudwatchevents::Client, source: &str, detail: &str) {
    println!("Publishing event:");
    println!("Source: {}", source);
    println!("Detail: {}", detail);

    let output = client
        .put_events()
        .entries(
            PutEventsRequestEntry::builder()
                .source(source)
                .detail(detail)
                .detail_type("UserEvent")
                .build(),
        )
        .send()
        .await
        .expect("Failed to publish event");

    println!("Output: {:?}", output);
}

pub fn subscribe(source: &str) {
    println!("Subscribing to event:");
    println!("Source: {}", source);
}
