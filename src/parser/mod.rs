use anyhow::Result;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[grammar = "parser/reading.pest"]
struct ReadingParser;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadingPart {
    pub part: String,
    pub reading: Option<String>,
}

pub fn parse_reading_string(reading_str: &str) -> Result<Vec<ReadingPart>> {
    let parser: Pair<Rule> = ReadingParser::parse(Rule::parser, reading_str)?
        .next()
        .unwrap();

    let parts: Vec<ReadingPart> = parser
        .into_inner()
        .map(|outer_type: Pair<Rule>| match outer_type.as_rule() {
            Rule::furi_group_single => {
                let mut inner_rules: Pairs<Rule> = outer_type.into_inner();
                let kanji = inner_rules.next().unwrap().as_str();
                let reading = inner_rules.next().unwrap().into_inner().as_str();

                vec![ReadingPart {
                    part: kanji.into(),
                    reading: Some(reading.into()),
                }]
            }
            Rule::furi_group_multi => {
                let mut inner_rules: Pairs<Rule> = outer_type.into_inner();
                let kanji_chars: Vec<char> = inner_rules.next().unwrap().as_str().chars().collect();
                let readings: Vec<&str> = inner_rules.map(|r| r.into_inner().as_str()).collect();

                kanji_chars
                    .into_iter()
                    .enumerate()
                    .map(|(index, kanji_char)| {
                        let reading: &str = readings.get(index).unwrap_or(&"");

                        ReadingPart {
                            part: kanji_char.into(),
                            reading: Some(reading.into()),
                        }
                    })
                    .collect()
            }
            Rule::kana_str => {
                let kana: String = outer_type
                    .into_inner()
                    .map(|r| r.as_str())
                    .collect::<Vec<&str>>()
                    .join("");

                vec![ReadingPart {
                    part: kana.into(),
                    reading: None,
                }]
            }
            _ => panic!("can't happen here"),
        })
        .flatten()
        .collect();

    Ok(parts)
}
