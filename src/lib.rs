use anyhow::Result;
use serde_json::json;
use types::words::{WordsQueryOptions, WordsResponse};

pub mod parser;
pub mod types;

pub async fn words(
    client: &reqwest::Client,
    query: &str,
    options: Option<WordsQueryOptions>,
) -> Result<WordsResponse> {
    let WordsQueryOptions {
        language,
        no_english,
    } = options.unwrap_or_default();

    let resp = client
        .post("https://jotoba.de/api/search/words")
        .json(&json!({
          "query": query,
          "language": language,
          "no_english": no_english
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(resp)
}
