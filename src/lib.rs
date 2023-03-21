use anyhow::*;
use rss::Channel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub title: String,
    pub link: String,
    pub description: String,
    pub author: String,
}

impl From<rss::Item> for Item {
    fn from(value: rss::Item) -> Self {
        Self {
            title: value.title.unwrap_or("".into()),
            link: value.link.unwrap_or("".into()),
            description: value.description.unwrap_or("".into()),
            author: value.author.unwrap_or("".into()),
        }
    }
}

async fn parse(contents: &[u8]) -> Result<Vec<Item>> {
    let channel = Channel::read_from(&contents[..])?;
    let items: Vec<Item> = channel.items.into_iter().map(|item| item.into()).collect();
    Ok(items)
}

pub async fn read_from_url(url: &str) -> Result<Vec<Item>> {
    let contents = reqwest::get(url).await?.bytes().await?;
    Ok(parse(&contents[..]).await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    #[tokio::test]
    async fn it_works() -> Result<()> {
        let path = Path::new("./fixtures/zenn.xml");
        let mut file = File::open(path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        let items = parse(&buf).await?;
        let json = serde_json::to_string_pretty(&items)?;
        insta::assert_snapshot!(json);
        Ok(())
    }
}
