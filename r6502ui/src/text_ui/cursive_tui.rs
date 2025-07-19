use crate::text_ui::export_list_info::ExportListInfo;
use cursive::align::HAlign;
use cursive::backends::crossterm::crossterm::event::{
    Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers,
};
use cursive::event::{Callback, Event, EventResult, EventTrigger, Key};
use cursive::theme::{BaseColor, Color, ColorStyle, ColorType};
use cursive::view::{Finder, Nameable, Resizable, ScrollStrategy, Scrollable, Selector};
use cursive::views::{
    EditView, Layer, LinearLayout, NamedView, Panel, ResizedView, ScrollView, TextView,
};
use cursive::{Cursive, CursiveRunnable, CursiveRunner, View};
use r6502core::AddressRange;
use r6502cpu::Reg;
use r6502cpu::symbols::MapFile;
use r6502lib::emulator::{InstructionInfo, IoEvent};
use r6502lib::messages::{Command, DebugMessage, IoMessage, MonitorMessage, State};
use std::fmt::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};

const RIGHT_NAME: &str = "right";
const CURRENT_NAME: &str = "current";
const DISASSEMBLY_NAME: &str = "disassembly";
const STATE_NAME: &str = "state";
const STDOUT_NAME: &str = "stdout";
const STDOUT_CONTAINER_NAME: &str = "stdout-container";
const SYMBOLS_NAME: &str = "symbols";
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
        io_tx: &Sender<IoEvent>,
        map_file: MapFile,
    ) -> Self {
        let export_list_info = ExportListInfo::new(&map_file);
        let mut cursive = cursive::default().into_runner();
        cursive.add_fullscreen_layer(
            LinearLayout::horizontal()
                .child(Self::make_left())
                .child(Self::make_right(debug_tx, &export_list_info)),
        );
        cursive.set_fps(30);
        Self::add_global_callbacks(&mut cursive, debug_tx, export_list_info);
        Self::set_pre_event_inner_handler(&mut cursive, io_tx);
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

    fn step(&mut self) -> bool {
        use r6502lib::messages::IoMessage::WriteChar;
        use r6502lib::messages::MonitorMessage::{
            AfterExecute, BeforeExecute, FetchMemoryResponse, NotifyInvalidBrk, NotifyState,
        };

        if !self.cursive.is_running() {
            return false;
        }

        while let Some(message) = self.io_rx.try_iter().next() {
            match message {
                WriteChar(ch) => {
                    if ch == '\n' || !ch.is_ascii_control() {
                        self.cursive
                            .find_name::<TextView>(STDOUT_NAME)
                            .expect("Must exist")
                            .append(ch);
                    } else {
                        self.cursive
                            .find_name::<TextView>(STDOUT_NAME)
                            .expect("Must exist")
                            .append(format!("[?{}?]", ch as u8));
                    }
                }
            }
        }

        while let Some(message) = self.monitor_rx.try_iter().next() {
            match message {
                NotifyState(state) => self.on_notify_state(state),
                NotifyInvalidBrk => self.on_notify_invalid_brk(),
                BeforeExecute {
                    total_cycles,
                    reg,
                    instruction_info,
                } => self.on_before_execute(total_cycles, &reg, &instruction_info),
                AfterExecute {
                    total_cycles,
                    reg,
                    instruction_info,
                } => self.on_after_execute(total_cycles, &reg, &instruction_info),
                FetchMemoryResponse {
                    address_range,
                    snapshot,
                } => self.on_fetch_memory_response(&address_range, &snapshot),
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

    fn on_notify_state(&mut self, state: State) {
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

    fn on_notify_invalid_brk(&mut self) {
        self.cursive
            .find_name::<TextView>(COMMAND_FEEDBACK_NAME)
            .expect("Must exist")
            .set_content("Invalid software interrupt");
    }

    fn on_before_execute(
        &mut self,
        total_cycles: u64,
        reg: &Reg,
        instruction_info: &InstructionInfo,
    ) {
        self.cursive
            .find_name::<TextView>(REGISTERS_NAME)
            .expect("Must exist")
            .set_content(reg.to_string());
        self.cursive
            .find_name::<TextView>(CYCLES_NAME)
            .expect("Must exist")
            .set_content(format!("{total_cycles}"));
        self.update_current(Some(instruction_info));
    }

    fn on_after_execute(
        &mut self,
        total_cycles: u64,
        reg: &Reg,
        instruction_info: &InstructionInfo,
    ) {
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

    fn on_fetch_memory_response(&mut self, address_range: &AddressRange, snapshot: &[u8]) {
        let s = Self::format_snapshot(address_range, snapshot);
        self.cursive
            .find_name::<TextView>(COMMAND_RESPONSE_NAME)
            .expect("Must exist")
            .append(s);
    }

    fn format_snapshot(address_range: &AddressRange, bytes: &[u8]) -> String {
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

    fn panel<V>(view: V, label: &str) -> Panel<V> {
        Panel::new(view).title(label).title_position(HAlign::Left)
    }

    fn make_left() -> LinearLayout {
        let current = TextView::new("").with_name(CURRENT_NAME).fixed_height(1);
        let registers = TextView::new("").with_name(REGISTERS_NAME);
        let cycles = TextView::new("").with_name(CYCLES_NAME);
        let disassembly = TextView::new("")
            .with_name(DISASSEMBLY_NAME)
            .full_height()
            .scrollable()
            .scroll_strategy(ScrollStrategy::StickToBottom);
        let state = TextView::new("")
            .with_name(STATE_NAME)
            .fixed_height(1)
            .scrollable()
            .scroll_strategy(ScrollStrategy::StickToBottom);

        LinearLayout::vertical()
            .child(Self::panel(current, "Current Instruction"))
            .child(Self::panel(registers, "Registers"))
            .child(Self::panel(cycles, "Cycles"))
            .child(Self::panel(disassembly, "Disassembly"))
            .child(Self::panel(state, "Status"))
    }

    fn make_right(
        debug_tx: &Sender<DebugMessage>,
        export_list_info: &ExportListInfo,
    ) -> NamedView<LinearLayout> {
        let stdout = TextView::new("")
            .style(STDOUT_TEXT_COLOUR_INACTIVE)
            .with_name(STDOUT_NAME)
            .full_width()
            .full_height()
            .scrollable()
            .scroll_strategy(ScrollStrategy::StickToBottom);
        let stdout_container = Layer::new(stdout).with_name(STDOUT_CONTAINER_NAME);
        let symbols = TextView::new(export_list_info.toggle())
            .with_name(SYMBOLS_NAME)
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
            S: Toggle symbol sort order\n\
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
            .on_submit(move |c, text| {
                c.call_on_name(COMMAND_NAME, |command: &mut EditView| {
                    _ = command.set_content("");
                });
                Self::run_command(c, text, &d);
            })
            .with_name(COMMAND_NAME)
            .fixed_height(1);
        let command_feedback = TextView::new("")
            .with_name(COMMAND_FEEDBACK_NAME)
            .fixed_height(1);

        LinearLayout::vertical()
            .child(Self::panel(stdout_container, "Output"))
            .child(Self::panel(symbols, "Symbols"))
            .child(Self::panel(help, "Help"))
            .child(Self::panel(command_response, "Command Response"))
            .child(Self::panel(command, "Command"))
            .child(Panel::new(command_feedback))
            .with_name(RIGHT_NAME)
    }

    fn add_global_callbacks(
        c: &mut Cursive,
        debug_tx: &Sender<DebugMessage>,
        export_list_info: ExportListInfo,
    ) {
        use r6502lib::messages::DebugMessage::{Break, Run, Step};

        c.add_global_callback('q', Cursive::quit);
        c.add_global_callback('s', move |c| {
            c.call_on_name(SYMBOLS_NAME, |view: &mut TextView| {
                view.set_content(export_list_info.toggle());
            })
            .unwrap();
        });
        let d = debug_tx.clone();
        c.add_global_callback(' ', move |_| _ = d.send(Step));
        let d = debug_tx.clone();
        c.add_global_callback('r', move |_| _ = d.send(Run));
        let d = debug_tx.clone();
        c.add_global_callback('b', move |_| _ = d.send(Break));
        c.add_global_callback('c', move |c| {
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
        c.add_global_callback(Key::Esc, move |c| {
            c.call_on_name(COMMAND_NAME, |command: &mut EditView| {
                command.disable();
            });
        });
    }

    fn set_pre_event_inner_handler(
        c: &mut CursiveRunner<CursiveRunnable>,
        io_tx: &Sender<IoEvent>,
    ) {
        let program_has_input = AtomicBool::new(false);
        let io_tx_clone = io_tx.clone();
        c.set_on_pre_event_inner(EventTrigger::any(), move |e| {
            fn send_char(io_tx: &Sender<IoEvent>, ch: char) {
                _ = io_tx.send(IoEvent::Input(CrosstermEvent::Key(KeyEvent::new(
                    KeyCode::Char(ch),
                    KeyModifiers::NONE,
                ))));
            }

            fn send_ctrl_char(io_tx: &Sender<IoEvent>, ch: char) {
                _ = io_tx.send(IoEvent::Input(CrosstermEvent::Key(KeyEvent::new(
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

            if program_has_input.load(Ordering::SeqCst) {
                match e {
                    Event::CtrlChar('p') => {
                        program_has_input.store(false, Ordering::SeqCst);
                        return Some(EventResult::Consumed(Some(Callback::from_fn(|c| {
                            set_stdout_colour(c, false);
                        }))));
                    }
                    // TBD: Get this working!
                    Event::CtrlChar('r') => send_ctrl_char(&io_tx_clone, 'r'),
                    // TBD: Get this working!
                    Event::CtrlChar('s') => send_ctrl_char(&io_tx_clone, 's'),
                    Event::Key(Key::Backspace | Key::Del) => send_char(&io_tx_clone, '_'),
                    Event::Key(Key::Enter) => send_char(&io_tx_clone, '\r'),
                    Event::Key(Key::Esc) => send_char(&io_tx_clone, 0x1b as char),
                    Event::Char(ch) => send_char(&io_tx_clone, *ch),
                    _ => return None,
                }
                Some(EventResult::consumed())
            } else {
                match e {
                    Event::CtrlChar('p') => {
                        program_has_input.store(true, Ordering::SeqCst);
                        Some(EventResult::Consumed(Some(Callback::from_fn(|c| {
                            set_stdout_colour(c, true);
                        }))))
                    }
                    _ => None,
                }
            }
        });
    }

    fn run_command(c: &mut Cursive, text: &str, d: &Sender<DebugMessage>) {
        use r6502lib::messages::DebugMessage::{FetchMemory, Go, SetPc};

        match text.parse::<Command>() {
            Ok(Command::Help(help)) => {
                c.find_name::<TextView>(COMMAND_RESPONSE_NAME)
                    .expect("Must exist")
                    .append(help);
            }
            Ok(Command::FetchMemory(address_range)) => {
                _ = d.send(FetchMemory(address_range));
                c.call_on_name(COMMAND_NAME, |command: &mut EditView| {
                    command.disable();
                });
            }
            Ok(Command::SetPc(addr)) => {
                _ = d.send(SetPc(addr));
                c.call_on_name(COMMAND_NAME, |command: &mut EditView| {
                    command.disable();
                });
            }
            Ok(Command::Go(addr)) => {
                _ = d.send(Go(addr));
                c.call_on_name(COMMAND_NAME, |command: &mut EditView| {
                    command.disable();
                });
            }
            Err(e) => {
                c.call_on_name(COMMAND_FEEDBACK_NAME, |view: &mut TextView| {
                    view.set_content(format!("{e}"));
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::text_ui::cursive_tui::CursiveTui;
    use anyhow::Result;
    use r6502core::AddressRange;

    #[test]
    fn format_snapshot() -> Result<()> {
        let address_range = AddressRange::new(0x0e00, 0x0e20)?;
        let bytes = (0x00..=0x20).collect::<Vec<_>>();
        let s = CursiveTui::format_snapshot(&address_range, &bytes);
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
