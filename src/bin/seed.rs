use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let count = std::env::args()
        .nth(1)
        .context("Usage: seed <count>")?
        .parse()
        .context("count must be a whole number")?;
    rust_starter::seed(count).await
}
