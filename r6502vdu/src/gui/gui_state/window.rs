use crate::gui::WindowState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Window {
    #[serde(rename = "state")]
    pub state: WindowState,

    #[serde(rename = "x")]
    pub x: i32,

    #[serde(rename = "y")]
    pub y: i32,

    #[serde(rename = "width")]
    pub width: u32,

    #[serde(rename = "height")]
    pub height: u32,
}
