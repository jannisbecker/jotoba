#[tokio::main]
pub async fn main() {
    let client = reqwest::Client::new();
    let result = jotoba_client::words(&client, "こんにちは", None)
        .await
        .unwrap();

    let word = result.words.get(0).unwrap();
    let furi = word.reading.furigana.clone();

    println!("{:#?}", furi);

    if let Some(furi) = furi {
        let readings = jotoba_client::parser::parse_reading_string(furi.as_str());
        println!("{:#?}", readings);
    }
}
