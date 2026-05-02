use thiserror::Error;


#[derive(Debug, Error)]
pub enum YahooErrors {
    /// An error originating from the underlying HTTP client (`reqwest`).
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// An error during JSON serialization or deserialization.
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    /// An error that occurs when parsing a URL.
    #[error("Invalid URL: {0}")]
    Url(#[from] url::ParseError),

    /// A 404 Not Found returned by Yahoo endpoints.
    #[error("Not found at {url}")]
    NotFound {
        /// The URL that returned a 404.
        url: String,
    },

    /// A 429 Too Many Requests (rate limit) returned by Yahoo endpoints.
    #[error("Rate limited at {url}")]
    RateLimited {
        /// The URL that returned a 429.
        url: String,
    },

    /// A 5xx server error returned by Yahoo endpoints.
    #[error("Server error {status} at {url}")]
    ServerError {
        /// The HTTP status code in the 5xx range.
        status: u16,
        /// The URL that returned a server error.
        url: String,
    },

    /// An error indicating an unexpected, non-successful HTTP status code (non-404/429/5xx).
    #[error("Unexpected response status: {status} at {url}")]
    Status {
        /// The unexpected HTTP status code returned.
        status: u16,
        /// The URL that returned the status.
        url: String,
    },

    /// An error returned by the Yahoo Finance API within an otherwise successful response.
    ///
    /// For example, a `200 OK` response might contain a JSON body with an `error` field.
    #[error("Yahoo API error: {0}")]
    Api(String),

    /// An error related to authentication, such as failing to retrieve a cookie or crumb.
    #[error("Authentication error: {0}")]
    Auth(String),

    /// An error that occurs during the web scraping process.
    #[error("Web scraping error: {0}")]
    Scrape(String),

    /// Indicates that an expected piece of data was missing from the API response.
    #[error("Missing data in response: {0}")]
    MissingData(String),

    /// An error indicating that the parameters provided by the caller were invalid.
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    /// An error indicating that the provided date range is invalid (e.g., start date after end date).
    #[error("Invalid date range: start date must be before end date")]
    InvalidDates,
}