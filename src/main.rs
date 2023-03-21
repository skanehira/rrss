use anyhow::*;
use rrss::read_from_url;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Please specifiy URL");
    }

    let url = &args[1];
    let items = read_from_url(url).await?;
    let json = serde_json::to_string_pretty(&items)?;
    println!("{}", json);
    Ok(())
}
