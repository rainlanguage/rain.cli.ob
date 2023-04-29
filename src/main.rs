mod cli;
pub mod subgraph;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::main().await
}