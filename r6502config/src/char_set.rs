use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub enum CharSet {
    #[default]
    #[serde(rename = "default")]
    Default,

    #[serde(rename = "acorn")]
    Acorn,

    #[serde(rename = "apple1")]
    Apple1,
}
