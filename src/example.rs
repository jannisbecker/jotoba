#[tokio::main]
pub async fn main() {
    let client = reqwest::Client::new();

    let words = jotoba::words(&client, "こんにちは", None).await.unwrap();

    let word = words.words.get(0).unwrap();
    let furi = word.reading.furigana.clone().unwrap();

    println!("{:#?}", furi);

    let readings = jotoba::parser::parse_reading_string(furi.as_str());

    println!("{:#?}", readings);
}
