use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let usage = "Usage: seed <email> <count>";
    let email = args.next().context(usage)?;
    let count = args
        .next()
        .context(usage)?
        .parse()
        .context("count must be a whole number")?;
    rust_starter::seed(&email, count).await
}
