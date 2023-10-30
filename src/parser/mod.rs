use anyhow::Result;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use serde::{Serialize, Deserialize};

#[derive(Parser)]
#[grammar = "parser/reading.pest"]
struct ReadingParser;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadingPart {
    pub part: String,
    pub reading: Option<ReadingType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReadingType {
    Combined(String),
    Separate(Vec<String>),
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

                ReadingPart {
                    part: kanji.into(),
                    reading: Some(ReadingType::Combined(reading.into())),
                }
            }
            Rule::furi_group_multi => {
                let mut inner_rules: Pairs<Rule> = outer_type.into_inner();
                let kanji: &str = inner_rules.next().unwrap().as_str();
                let readings: Vec<String> = inner_rules
                    .map(|r| r.into_inner().as_str().into())
                    .collect();

                ReadingPart {
                    part: kanji.into(),
                    reading: Some(ReadingType::Separate(readings)),
                }
            }
            Rule::kana_str => {
                let kana: &str = outer_type.into_inner().next().unwrap().as_str();
                ReadingPart {
                    part: kana.into(),
                    reading: None,
                }
            }
            _ => panic!("can't happen here"),
        })
        .collect();

    Ok(parts)
}
