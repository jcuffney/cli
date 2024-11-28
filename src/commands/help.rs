use clap::Args;

#[derive(Args, Debug)]
pub struct HelpCommand {
    /// Display detailed help
    #[arg(short, long)]
    pub detailed: bool,
}
