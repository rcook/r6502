use crate::gui::display_util::get_default_bounds;
use crate::gui::{State, WindowState};
use anyhow::Result;
use sdl3::video::{FullscreenType, Window as Sdl3Window};
use std::fs::File;
use std::path::{Path, PathBuf};

pub struct StateManager {
    path: PathBuf,
    state: Option<State>,
}

impl StateManager {
    pub fn new(path: &Path) -> Result<Self> {
        let state = if path.is_file() {
            let f = File::open(path)?;
            serde_json::from_reader::<_, State>(f).ok()
        } else {
            None
        };
        Ok(Self {
            path: path.to_path_buf(),
            state,
        })
    }

    pub fn state(&self) -> Option<&State> {
        self.state.as_ref()
    }

    pub fn update(&mut self, window: &Sdl3Window) -> Result<()> {
        let state: &mut State = if let Some(state) = self.state.as_mut() {
            state
        } else {
            self.state = Some(State::default());
            self.state.as_mut().unwrap()
        };

        match window.fullscreen_state() {
            FullscreenType::Desktop => todo!(),
            FullscreenType::True => state.window.state = WindowState::FullScreen,
            FullscreenType::Off => {
                if window.is_maximized() {
                    assert!(!window.is_minimized());
                    state.window.state = WindowState::Maximized;
                } else if window.is_minimized() {
                    assert!(!window.is_maximized());
                    state.window.state = WindowState::Minimized;
                } else {
                    state.window.state = WindowState::Normal;
                    let (left, top) = window.position();
                    state.window.x = left;
                    state.window.y = top;
                    let (width, height) = window.size();
                    state.window.width = width;
                    state.window.height = height;
                }
            }
        }

        if state.window.width == 0 || state.window.height == 0 {
            let bounds = get_default_bounds(window.get_display()?)?;
            state.window.x = bounds.x();
            state.window.y = bounds.y();
            state.window.width = bounds.width();
            state.window.height = bounds.height();
        }

        let f = File::create(&self.path)?;
        serde_json::to_writer_pretty(f, state)?;
        Ok(())
    }
}
