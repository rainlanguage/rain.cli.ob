use anyhow::Result;
use clap::command;
use clap::Command;
use clap::ArgMatches;

mod order;

pub async fn dispatch(matches: &ArgMatches) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches(order::NAME) {
        Ok(order::ls(matches).await?)
    }
    else {
        Ok(())
    }
}

pub async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::fmt::Subscriber::new())?;

    let matches = command!()
        .subcommand(Command::new(order::NAME).about(order::ABOUT))
        .get_matches();

    dispatch(&matches).await
}
