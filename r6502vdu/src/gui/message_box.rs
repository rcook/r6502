use anyhow::Result;
use sdl3::messagebox::{
    ButtonData, ClickedButton, MessageBoxButtonFlag, MessageBoxFlag, show_message_box,
};
use sdl3::video::Window;

const YES_BUTTON_ID: i32 = 0;
const NO_BUTTON_ID: i32 = 1;

pub fn confirm(window: Option<&Window>, title: &str, message: &str) -> Result<bool> {
    let yes_button = ButtonData {
        flags: MessageBoxButtonFlag::RETURNKEY_DEFAULT,
        button_id: YES_BUTTON_ID,
        text: "Yes",
    };
    let no_button = ButtonData {
        flags: MessageBoxButtonFlag::ESCAPEKEY_DEFAULT,
        button_id: NO_BUTTON_ID,
        text: "No",
    };
    let buttons = &[yes_button, no_button];
    let result = show_message_box(MessageBoxFlag::ERROR, buttons, title, message, window, None)?;
    Ok(matches!(result, ClickedButton::CustomButton(d) if d.button_id == YES_BUTTON_ID))
}
