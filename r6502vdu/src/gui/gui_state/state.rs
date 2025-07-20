use crate::gui::Window;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    #[serde(rename = "window")]
    pub window: Window,
}
