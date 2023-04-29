use clap::ArgMatches;

pub const NAME: &str = "order";
pub const ABOUT: &str = "Interact with an order(s) onchain.";

pub async fn ls(_matches: &ArgMatches) -> anyhow::Result<()> {
    tracing::info!("foo");
    crate::subgraph::orders::query().await?;
    Ok(())
}