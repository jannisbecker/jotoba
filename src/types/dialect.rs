use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Dialect {
        Brazilian,
        Hokkaido,
        Kansai,
        Kantou,
        Kyoto,
        Kyuushuu,
        Nagano,
        Osaka,
        Ryuukyuu,
        Touhoku,
        Tosa,
        Tsugaru,
}