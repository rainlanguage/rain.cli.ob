use anyhow::Result;
use clap::command;
use clap::Command;

mod order;

pub async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::fmt::Subscriber::new())?;

    let matches = command!()
        .subcommand(Command::new(order::NAME).about(order::ABOUT))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches(order::NAME) {
        order::ls(matches).await?;
    }

    Ok(())
}
