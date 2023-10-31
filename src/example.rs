#[tokio::main]
pub async fn main() {
    let client = reqwest::Client::new();
    let result = jotoba_client::words(&client, "オート", None).await.unwrap();

    let word = result.words.get(0).unwrap();
    let reading_str = word
        .reading
        .furigana
        .clone()
        .or_else(|| Some(word.reading.kana.clone()))
        .unwrap_or_default();
    let readings = jotoba_client::parser::parse_reading_string(&reading_str);

    println!("{:#?}", readings);
}
