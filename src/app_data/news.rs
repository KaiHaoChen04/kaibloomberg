use rss::Channel;
use crate::utils::YahooErrors;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum News {
    Tech,
    Finance,
}

impl News {
    pub const fn all() -> [News; 2] {
        [News::Finance, News::Tech]
    }

    pub const fn url(self) -> &'static str {
        match self {
            News::Finance => "https://finance.yahoo.com/news/rssindex",
            News::Tech => "https://news.yahoo.com/rss/tech",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            News::Finance => "World Finance",
            News::Tech => "Technology",
        }
    }
}

#[derive(Clone, Debug)]
pub struct NewsItem {
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub pub_date: Option<String>,
}

pub async fn fetch_news(source: News) -> Result<Vec<NewsItem>, YahooErrors> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .build()
        .map_err(YahooErrors::Http)?;

    let endpoint = source.url();
    let response = client
        .get(endpoint)
        .send()
        .await
        .map_err(YahooErrors::Http)?;

    let status = response.status();
    let body = response.bytes().await.map_err(YahooErrors::Http)?;

    if !status.is_success() {
        return Err(match status.as_u16() {
            404 => YahooErrors::NotFound {
                url: endpoint.to_string(),
            },
            429 => YahooErrors::RateLimited {
                url: endpoint.to_string(),
            },
            500..=599 => YahooErrors::ServerError {
                status: status.as_u16(),
                url: endpoint.to_string(),
            },
            _ => YahooErrors::Status {
                status: status.as_u16(),
                url: endpoint.to_string(),
            },
        });
    }

    let channel = Channel::read_from(&body[..]).map_err(|error| {
        YahooErrors::Scrape(format!(
            "{} feed parse failed at {}: {}",
            source.label(),
            endpoint,
            error
        ))
    })?;

    parse_channel(source, endpoint, &channel)
}

fn parse_channel(
    source: News,
    endpoint: &str,
    channel: &Channel,
) -> Result<Vec<NewsItem>, YahooErrors> {
    let items: Vec<NewsItem> = channel
        .items()
        .iter()
        .filter_map(|item| {
            let title = item.title()?.trim();
            let link = item.link()?.trim();

            if title.is_empty() || link.is_empty() {
                return None;
            }

            Some(NewsItem {
                title: title.to_string(),
                link: link.to_string(),
                description: item
                    .description()
                    .map(|value| value.trim().to_string())
                    .filter(|value| !value.is_empty()),
                pub_date: item
                    .pub_date()
                    .map(|value| value.trim().to_string())
                    .filter(|value| !value.is_empty()),
            })
        })
        .collect();

    if items.is_empty() {
        return Err(YahooErrors::MissingData(format!(
            "No usable RSS items returned for {} at {}",
            source.label(),
            endpoint
        )));
    }

    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::{News, parse_channel};
    use rss::{ChannelBuilder, ItemBuilder};

    #[test]
    fn news_sources_have_labels_and_urls() {
        assert_eq!(News::Finance.label(), "World Finance");
        assert_eq!(News::Tech.label(), "Technology");
        assert!(News::Finance.url().contains("finance.yahoo.com"));
        assert!(News::Tech.url().contains("news.yahoo.com"));
    }

    #[test]
    fn parse_channel_skips_sparse_items() {
        let channel = ChannelBuilder::default()
            .title("Test".to_string())
            .items(vec![
                ItemBuilder::default()
                    .title(Some("Headline".to_string()))
                    .link(Some("https://example.com/story".to_string()))
                    .build(),
                ItemBuilder::default().build(),
            ])
            .build();

        let items = parse_channel(News::Finance, "https://example.com/rss", &channel)
            .expect("usable item should parse");

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "Headline");
        assert_eq!(items[0].link, "https://example.com/story");
    }
}
