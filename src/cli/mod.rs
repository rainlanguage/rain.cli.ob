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

    let root_command = command!();
    let ob_command = Command::new("orderbook").about("orderbook stuff");
    let order_command = Command::new(order::NAME).about(order::ABOUT);

    ob_command.subcommand(order_command);
    root_command.subcommand(ob_command);

    let matches = order_command.get_matches();

    dispatch(&matches).await
}
