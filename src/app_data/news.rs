use std::error::Error;
use rss::Channel;

pub enum News {
    Tech,
    Finance,
}

impl News {
    fn url(&self) -> &'static str {
        match self {
            News::Finance => "https://finance.yahoo.com/news/rssindex",
            News::Tech => "https://news.yahoo.com/rss/tech",
        }
    }
    fn label(&self) -> &'static str {
        match self {
            News::Finance => "World Finance",
            News::Tech => "Technology",
        }
    }
}

async fn fetch_and_parse_title(source: News) -> Result<Vec<String>, Box<dyn Error>> {
    let response = reqwest::get(source.url()).await?.bytes().await?;
    let channel = Channel::read_from(&response[..])?;

    let titles = channel
        .items()
        .iter()
        .filter_map(|item| item.title().map(|t| t.to_string()))
        .collect();

    Ok(titles)

}
