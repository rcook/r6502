use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum HostHookType {
    #[serde(rename = "acorn")]
    Acorn,
}
