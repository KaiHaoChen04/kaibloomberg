
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
