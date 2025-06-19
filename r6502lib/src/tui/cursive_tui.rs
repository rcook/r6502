use crate::emulator::{AddressRange, InstructionInfo, PiaEvent};
use crate::messages::{Command, DebugMessage, IoMessage, MonitorMessage, State};
use crate::symbols::{Export, MapFile};
use cursive::align::HAlign;
use cursive::backends::crossterm::crossterm::event::{
    Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers,
};
use cursive::direction::Orientation;
use cursive::event::{Callback, Event, EventResult, EventTrigger, Key};
use cursive::theme::{BaseColor, Color, ColorStyle, ColorType};
use cursive::view::{Finder, Nameable, Resizable, ScrollStrategy, Scrollable, Selector};
use cursive::views::{
    EditView, Layer, LinearLayout, NamedView, Panel, ResizedView, ScrollView, TextView,
};
use cursive::{Cursive, CursiveRunnable, CursiveRunner, View};
use std::fmt::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};

const RIGHT_NAME: &str = "right";
const CURRENT_NAME: &str = "current";
const DISASSEMBLY_NAME: &str = "disassembly";
const STATE_NAME: &str = "state";
const STDOUT_NAME: &str = "stdout";
const STDOUT_CONTAINER_NAME: &str = "stdout-container";
const REGISTERS_NAME: &str = "registers";
const CYCLES_NAME: &str = "cycles";
const COMMAND_RESPONSE_NAME: &str = "command-response";
const COMMAND_NAME: &str = "command";
const COMMAND_FEEDBACK_NAME: &str = "command-feedback";

const STDOUT_TEXT_COLOUR_ACTIVE: Color = Color::Light(BaseColor::Yellow);
const STDOUT_TEXT_COLOUR_INACTIVE: Color = Color::Dark(BaseColor::Blue);
const STDOUT_BACKGROUND_COLOUR_ACTIVE: ColorType = ColorType::Color(Color::Dark(BaseColor::Black));
const STDOUT_BACKGROUND_COLOUR_INACTIVE: ColorType =
    ColorType::Color(Color::Dark(BaseColor::White));

pub struct CursiveTui {
    cursive: CursiveRunner<CursiveRunnable>,
    monitor_rx: Receiver<MonitorMessage>,
    io_rx: Receiver<IoMessage>,
    map_file: MapFile,
}

impl CursiveTui {
    pub fn new(
        monitor_rx: Receiver<MonitorMessage>,
        io_rx: Receiver<IoMessage>,
        debug_tx: &Sender<DebugMessage>,
        pia_tx: &Sender<PiaEvent>,
        map_file: MapFile,
    ) -> Self {
        let cursive = Self::make_ui(&map_file, debug_tx, pia_tx);
        Self {
            cursive,
            monitor_rx,
            io_rx,
            map_file,
        }
    }

    pub fn run(&mut self) {
        while self.step() {}
    }

    #[allow(clippy::too_many_lines)]
    fn make_ui(
        map_file: &MapFile,
        debug_tx: &Sender<DebugMessage>,
        pia_tx: &Sender<PiaEvent>,
    ) -> CursiveRunner<CursiveRunnable> {
        use crate::messages::DebugMessage::{Break, FetchMemory, Go, Run, SetPc, Step};

        fn panel<V>(view: V, label: &str) -> Panel<V> {
            Panel::new(view).title(label).title_position(HAlign::Left)
        }

        let s = map_file
            .exports
            .iter()
            .map(Export::to_string)
            .collect::<Vec<_>>()
            .join("\n");

        let state = TextView::new("")
            .with_name(STATE_NAME)
            .fixed_height(1)
            .scrollable()
            .scroll_strategy(ScrollStrategy::StickToBottom);
        let current = TextView::new("").with_name(CURRENT_NAME).fixed_height(1);
        let registers = TextView::new("").with_name(REGISTERS_NAME);
        let cycles = TextView::new("").with_name(CYCLES_NAME);
        let disassembly = TextView::new("")
            .with_name(DISASSEMBLY_NAME)
            .full_height()
            .scrollable()
            .scroll_strategy(ScrollStrategy::StickToBottom);
        let stdout = TextView::new("")
            .style(STDOUT_TEXT_COLOUR_INACTIVE)
            .with_name(STDOUT_NAME)
            .full_width()
            .full_height()
            .scrollable()
            .scroll_strategy(ScrollStrategy::StickToBottom);
        let stdout_container = Layer::new(stdout).with_name(STDOUT_CONTAINER_NAME);
        let symbols = TextView::new(s)
            .min_height(10)
            .full_width()
            .scrollable()
            .scroll_strategy(ScrollStrategy::KeepRow);
        let help = TextView::new(
            "Q: Quit\n\
            Space: Step\n\
            R: Run\n\
            B: Break\n\
            C: Command\n\
            Esc: Exit command\n\
            Ctrl+P: Toggle between debugger and program input",
        )
        .full_width()
        .scrollable()
        .scroll_strategy(ScrollStrategy::KeepRow);
        let d = debug_tx.clone();
        let command_response = TextView::new("")
            .with_name(COMMAND_RESPONSE_NAME)
            .full_width()
            .min_height(5)
            .scrollable()
            .scroll_strategy(ScrollStrategy::StickToBottom);
        let command = EditView::new()
            .disabled()
            .on_submit(move |s, text| {
                fn nasty_hack(s: &mut Cursive, text: &str, d: &Sender<DebugMessage>) {
                    match text.parse::<Command>() {
                        Ok(Command::Help(help)) => {
                            s.find_name::<TextView>(COMMAND_RESPONSE_NAME)
                                .expect("Must exist")
                                .append(help);
                        }
                        Ok(Command::FetchMemory(address_range)) => {
                            _ = d.send(FetchMemory(address_range));
                            s.call_on_name(COMMAND_NAME, |command: &mut EditView| {
                                command.disable();
                            });
                        }
                        Ok(Command::SetPc(addr)) => {
                            _ = d.send(SetPc(addr));
                            s.call_on_name(COMMAND_NAME, |command: &mut EditView| {
                                command.disable();
                            });
                        }
                        Ok(Command::Go(addr)) => {
                            _ = d.send(Go(addr));
                            s.call_on_name(COMMAND_NAME, |command: &mut EditView| {
                                command.disable();
                            });
                        }
                        Err(e) => {
                            s.call_on_name(COMMAND_FEEDBACK_NAME, |view: &mut TextView| {
                                view.set_content(format!("{e}"));
                            });
                        }
                    }
                }
                s.call_on_name(COMMAND_NAME, |command: &mut EditView| {
                    _ = command.set_content("");
                });
                nasty_hack(s, text, &d);
            })
            .with_name(COMMAND_NAME)
            .fixed_height(1);
        let command_feedback = TextView::new("")
            .with_name(COMMAND_FEEDBACK_NAME)
            .fixed_height(1);

        let layout1 = LinearLayout::horizontal()
            .child(panel(symbols, "Symbols"))
            .child(panel(help, "Help"));
        let layout2 = LinearLayout::vertical()
            .child(layout1)
            .child(panel(command_response, "Command Response"));

        let left = LinearLayout::new(Orientation::Vertical)
            .child(panel(current, "Current Instruction"))
            .child(panel(registers, "Registers"))
            .child(panel(cycles, "Cycles"))
            .child(panel(disassembly, "Disassembly"))
            .child(panel(state, "Status"));

        let right = LinearLayout::new(Orientation::Vertical)
            .child(panel(stdout_container, "Output"))
            .child(layout2)
            .child(panel(command, "Command"))
            .child(Panel::new(command_feedback))
            .with_name(RIGHT_NAME);

        let dashboard = LinearLayout::new(Orientation::Horizontal)
            .child(left)
            .child(right);

        let mut cursive = cursive::default().into_runner();
        cursive.add_fullscreen_layer(dashboard);

        cursive.set_fps(30);

        cursive.add_global_callback('q', Cursive::quit);
        let d = debug_tx.clone();
        cursive.add_global_callback(' ', move |_| _ = d.send(Step));
        let d = debug_tx.clone();
        cursive.add_global_callback('r', move |_| _ = d.send(Run));
        let d = debug_tx.clone();
        cursive.add_global_callback('b', move |_| _ = d.send(Break));
        cursive.add_global_callback('c', move |c| {
            c.call_on_name(RIGHT_NAME, |right: &mut LinearLayout| {
                // https://github.com/gyscos/cursive/discussions/820#discussioncomment-13299361
                right
                    .call_on_name(COMMAND_NAME, EditView::enable)
                    .expect("Must succeed");
                right
                    .focus_view(&Selector::Name(COMMAND_NAME))
                    .expect("Must succeed");
            });
        });
        cursive.add_global_callback(Key::Esc, move |c| {
            c.call_on_name(COMMAND_NAME, |command: &mut EditView| {
                command.disable();
            });
        });

        let program_gets_input = AtomicBool::new(false);
        let pia_tx_clone = pia_tx.clone();
        cursive.set_on_pre_event_inner(EventTrigger::any(), move |e| {
            fn send_char(pia_tx: &Sender<PiaEvent>, ch: char) {
                _ = pia_tx.send(PiaEvent::Input(CrosstermEvent::Key(KeyEvent::new(
                    KeyCode::Char(ch),
                    KeyModifiers::NONE,
                ))));
            }

            fn send_ctrl_char(pia_tx: &Sender<PiaEvent>, ch: char) {
                _ = pia_tx.send(PiaEvent::Input(CrosstermEvent::Key(KeyEvent::new(
                    KeyCode::Char(ch),
                    KeyModifiers::CONTROL,
                ))));
            }

            fn set_stdout_colour(c: &mut Cursive, active: bool) {
                c.call_on_name(STDOUT_NAME, |stdout: &mut TextView| {
                    stdout.set_style(if active {
                        STDOUT_TEXT_COLOUR_ACTIVE
                    } else {
                        STDOUT_TEXT_COLOUR_INACTIVE
                    });
                });
                c.call_on_name(
                    STDOUT_CONTAINER_NAME,
                    |layer: &mut Layer<
                        ScrollView<ResizedView<ResizedView<NamedView<TextView>>>>,
                    >| {
                        layer.set_color(ColorStyle {
                            front: ColorType::Color(Color::TerminalDefault),
                            back: if active {
                                STDOUT_BACKGROUND_COLOUR_ACTIVE
                            } else {
                                STDOUT_BACKGROUND_COLOUR_INACTIVE
                            },
                        });
                    },
                );
            }

            if program_gets_input.load(Ordering::SeqCst) {
                match e {
                    Event::CtrlChar('p') => {
                        program_gets_input.store(false, Ordering::SeqCst);
                        return Some(EventResult::Consumed(Some(Callback::from_fn(|c| {
                            set_stdout_colour(c, false);
                        }))));
                    }
                    // TBD: Get this working!
                    Event::CtrlChar('r') => send_ctrl_char(&pia_tx_clone, 'r'),
                    // TBD: Get this working!
                    Event::CtrlChar('s') => send_ctrl_char(&pia_tx_clone, 's'),
                    Event::Key(Key::Backspace | Key::Del) => send_char(&pia_tx_clone, '_'),
                    Event::Key(Key::Enter) => send_char(&pia_tx_clone, '\r'),
                    Event::Key(Key::Esc) => send_char(&pia_tx_clone, 0x1b as char),
                    Event::Char(ch) => send_char(&pia_tx_clone, *ch),
                    _ => return None,
                }
                Some(EventResult::consumed())
            } else {
                match e {
                    Event::CtrlChar('p') => {
                        program_gets_input.store(true, Ordering::SeqCst);
                        Some(EventResult::Consumed(Some(Callback::from_fn(|c| {
                            set_stdout_colour(c, true);
                        }))))
                    }
                    _ => None,
                }
            }
        });

        cursive
    }

    #[allow(clippy::too_many_lines)]
    fn step(&mut self) -> bool {
        use crate::messages::IoMessage::WriteChar;
        use crate::messages::MonitorMessage::{
            AfterExecute, BeforeExecute, FetchMemoryResponse, NotifyInvalidBrk, NotifyState,
        };

        if !self.cursive.is_running() {
            return false;
        }

        while let Some(message) = self.io_rx.try_iter().next() {
            match message {
                WriteChar(ch) => {
                    if ch != '\r' {
                        self.cursive
                            .find_name::<TextView>(STDOUT_NAME)
                            .expect("Must exist")
                            .append(ch);
                    }
                }
            }
        }

        while let Some(message) = self.monitor_rx.try_iter().next() {
            match message {
                NotifyState(state) => {
                    let s = match state {
                        State::Running => "Running",
                        State::Stepping => "Stepping",
                        State::Halted => "Waiting",
                        State::Stopped => "Stopped",
                    };
                    self.cursive
                        .find_name::<TextView>(STATE_NAME)
                        .expect("Must exist")
                        .set_content(s);
                    if matches!(state, State::Halted | State::Running | State::Stopped) {
                        self.update_current(None);
                    }
                }
                NotifyInvalidBrk => {
                    self.cursive
                        .find_name::<TextView>(COMMAND_FEEDBACK_NAME)
                        .expect("Must exist")
                        .set_content("Invalid software interrupt");
                }
                BeforeExecute {
                    total_cycles,
                    reg,
                    instruction_info,
                } => {
                    self.cursive
                        .find_name::<TextView>(REGISTERS_NAME)
                        .expect("Must exist")
                        .set_content(reg.to_string());
                    self.cursive
                        .find_name::<TextView>(CYCLES_NAME)
                        .expect("Must exist")
                        .set_content(format!("{total_cycles}"));
                    self.update_current(Some(&instruction_info));
                }
                AfterExecute {
                    total_cycles,
                    reg,
                    instruction_info,
                } => {
                    self.cursive
                        .find_name::<TextView>(REGISTERS_NAME)
                        .expect("Must exist")
                        .set_content(reg.to_string());
                    self.cursive
                        .find_name::<TextView>(CYCLES_NAME)
                        .expect("Must exist")
                        .set_content(format!("{total_cycles}"));

                    let mut s = instruction_info
                        .disassembly(&self.map_file)
                        .expect("Must succeed");
                    s.push('\n');
                    self.cursive
                        .find_name::<TextView>(DISASSEMBLY_NAME)
                        .expect("Must exist")
                        .append(s);
                }
                FetchMemoryResponse {
                    address_range,
                    snapshot,
                } => {
                    let s = Self::format_snapshot(address_range, snapshot);
                    self.cursive
                        .find_name::<TextView>(COMMAND_RESPONSE_NAME)
                        .expect("Must exist")
                        .append(s);
                }
            }
        }

        self.cursive.step();
        true
    }

    fn update_current(&mut self, instruction_info: Option<&InstructionInfo>) {
        if let Some(instruction_info) = instruction_info {
            self.cursive
                .find_name::<TextView>(CURRENT_NAME)
                .expect("Must exist")
                .set_content(
                    instruction_info
                        .display(&self.map_file)
                        .expect("Must succeed"),
                );
        } else {
            self.cursive
                .find_name::<TextView>(CURRENT_NAME)
                .expect("Must exist")
                .set_content("(running)");
            self.cursive
                .find_name::<TextView>(REGISTERS_NAME)
                .expect("Must exist")
                .set_content("(running)");
            self.cursive
                .find_name::<TextView>(CYCLES_NAME)
                .expect("Must exist")
                .set_content("(running)");
            self.cursive
                .find_name::<TextView>(DISASSEMBLY_NAME)
                .expect("Must exist")
                .set_content(' ');
        }
    }

    fn format_snapshot(address_range: AddressRange, bytes: Vec<u8>) -> String {
        const CHUNK_SIZE: usize = 16;
        let mut s = format!("{address_range}\n");
        let mut addr = address_range.start() as usize;
        for chunk in bytes.chunks(CHUNK_SIZE) {
            write!(s, "{addr:04X} ").unwrap();
            let mut chars = String::with_capacity(CHUNK_SIZE);
            for b in chunk {
                write!(s, " {b:02X}").unwrap();
                let c: char = *b as char;
                if c.is_ascii() && !c.is_ascii_control() {
                    chars.push(c);
                } else {
                    chars.push('.');
                }
            }
            s.push_str(&String::from("   ").repeat(CHUNK_SIZE - chunk.len()));
            writeln!(s, "  {chars}").unwrap();
            addr += CHUNK_SIZE;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::{emulator::AddressRange, tui::cursive_tui::CursiveTui};
    use anyhow::Result;

    #[test]
    fn format_snapshot() -> Result<()> {
        let address_range = AddressRange::new(0x0e00, 0x0e20)?;
        let bytes = (0x00..=0x20).collect();
        let s = CursiveTui::format_snapshot(address_range, bytes);
        assert_eq!(
            "$0E00:$0E20\n\
            0E00  00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F  ................\n\
            0E10  10 11 12 13 14 15 16 17 18 19 1A 1B 1C 1D 1E 1F  ................\n\
            0E20  20                                                \n",
            s
        );
        Ok(())
    }
}
