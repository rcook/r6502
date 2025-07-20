use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum WindowState {
    #[serde(rename = "fullScreen", alias = "full_screen")]
    FullScreen,

    #[serde(rename = "maximized")]
    Maximized,

    #[serde(rename = "minimized")]
    Minimized,

    #[serde(rename = "normal")]
    #[default]
    Normal,
}
