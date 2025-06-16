use crate::emulator::InstructionInfo;
use crate::messages::{Command, DebugMessage, IoMessage, MonitorMessage, State};
use crate::symbols::SymbolInfo;
use cursive::align::HAlign;
use cursive::backends::crossterm::crossterm::event::{
    Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers,
};
use cursive::direction::Orientation;
use cursive::event::{Event, EventResult, EventTrigger, Key};
use cursive::theme::{BaseColor, Color};
use cursive::view::{Finder, Nameable, Resizable, ScrollStrategy, Scrollable, Selector};
use cursive::views::{EditView, LinearLayout, Panel, TextView};
use cursive::{Cursive, CursiveRunnable, CursiveRunner, View};
use log::info;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};

const RIGHT_NAME: &str = "right";
const CURRENT_NAME: &str = "current";
const DISASSEMBLY_NAME: &str = "disassembly";
const STATE_NAME: &str = "state";
const STDOUT_NAME: &str = "stdout";
const REGISTERS_NAME: &str = "registers";
const CYCLES_NAME: &str = "cycles";
const COMMAND_RESPONSE_NAME: &str = "command-response";
const COMMAND_NAME: &str = "command";
const COMMAND_FEEDBACK_NAME: &str = "command-feedback";

pub struct CursiveTui {
    cursive: CursiveRunner<CursiveRunnable>,
    monitor_rx: Receiver<MonitorMessage>,
    io_rx: Receiver<IoMessage>,
    symbols: Vec<SymbolInfo>,
}

impl CursiveTui {
    pub fn new(
        monitor_rx: Receiver<MonitorMessage>,
        io_rx: Receiver<IoMessage>,
        debug_tx: &Sender<DebugMessage>,
        event_tx: &Sender<CrosstermEvent>,
        symbols: Vec<SymbolInfo>,
    ) -> Self {
        let cursive = Self::make_ui(&symbols, debug_tx, event_tx);
        Self {
            cursive,
            monitor_rx,
            io_rx,
            symbols,
        }
    }

    pub fn run(&mut self) {
        while self.step() {}
    }

    #[allow(clippy::too_many_lines)]
    fn make_ui(
        symbols: &[SymbolInfo],
        debug_tx: &Sender<DebugMessage>,
        event_tx: &Sender<CrosstermEvent>,
    ) -> CursiveRunner<CursiveRunnable> {
        use crate::messages::DebugMessage::{Break, FetchMemory, Go, Run, SetPc, Step};

        fn panel<V>(view: V, label: &str) -> Panel<V> {
            Panel::new(view).title(label).title_position(HAlign::Left)
        }

        let s = symbols
            .iter()
            .map(|s| format!("{} = ${:04X}", s.name, s.value))
            .collect::<Vec<_>>()
            .join("\n");

        let state = TextView::new("")
            .with_name(STATE_NAME)
            .fixed_height(3)
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
            .style(Color::Dark(BaseColor::Blue))
            .with_name(STDOUT_NAME)
            .full_width()
            .full_height()
            .scrollable()
            .scroll_strategy(ScrollStrategy::StickToBottom);
        let symbols = TextView::new(s)
            .min_height(10)
            .scrollable()
            .scroll_strategy(ScrollStrategy::KeepRow);
        let help =
            TextView::new("Q: Quit\nSpace: Step\nR: Run\nB: Break\nC: Command\nEsc: Exit command\nCtrl+P: Toggle between debugger and program input")
                .scrollable()
                .scroll_strategy(ScrollStrategy::KeepRow);
        let d = debug_tx.clone();
        let command_response = TextView::new("")
            .with_name(COMMAND_RESPONSE_NAME)
            .full_width()
            .full_height()
            .scrollable()
            .scroll_strategy(ScrollStrategy::StickToBottom);
        let command = EditView::new()
            .disabled()
            .on_submit(move |s, text| {
                fn nasty_hack(s: &mut Cursive, text: &str, d: &Sender<DebugMessage>) {
                    match text.parse::<Command>() {
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
                nasty_hack(s, text, &d);
            })
            .with_name(COMMAND_NAME)
            .fixed_height(1);
        let command_feedback = TextView::new("")
            .with_name(COMMAND_FEEDBACK_NAME)
            .fixed_height(1);

        let left = LinearLayout::new(Orientation::Vertical)
            .child(panel(current, "Current Instruction"))
            .child(panel(registers, "Registers"))
            .child(panel(cycles, "Cycles"))
            .child(panel(disassembly, "Disassembly"))
            .child(panel(state, "Status"));

        let right = LinearLayout::new(Orientation::Vertical)
            .child(panel(stdout, "Output"))
            .child(panel(symbols, "Symbols"))
            .child(panel(help, "Help"))
            .child(panel(command_response, "Command Response"))
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
        let event_tx_clone = event_tx.clone();
        cursive.set_on_pre_event_inner(EventTrigger::any(), move |e| {
            if program_gets_input.load(Ordering::SeqCst) {
                fn send_char(event_tx: &Sender<CrosstermEvent>, ch: char) {
                    _ = event_tx.send(CrosstermEvent::Key(KeyEvent::new(
                        KeyCode::Char(ch),
                        KeyModifiers::NONE,
                    )));
                }

                fn send_ctrl_char(event_tx: &Sender<CrosstermEvent>, ch: char) {
                    _ = event_tx.send(CrosstermEvent::Key(KeyEvent::new(
                        KeyCode::Char(ch),
                        KeyModifiers::CONTROL,
                    )));
                }

                match e {
                    Event::CtrlChar('p') => {
                        program_gets_input.store(false, Ordering::SeqCst);
                    }
                    // TBD: Get this working!
                    Event::CtrlChar('r') => send_ctrl_char(&event_tx_clone, 'r'),
                    // TBD: Get this working!
                    Event::CtrlChar('s') => send_ctrl_char(&event_tx_clone, 's'),
                    Event::Key(Key::Backspace | Key::Del) => send_char(&event_tx_clone, '_'),
                    Event::Key(Key::Enter) => send_char(&event_tx_clone, '\r'),
                    Event::Key(Key::Esc) => send_char(&event_tx_clone, 0x1b as char),
                    Event::Char(ch) => send_char(&event_tx_clone, *ch),
                    _ => {}
                }
                Some(EventResult::consumed())
            } else {
                match e {
                    Event::CtrlChar('p') => {
                        program_gets_input.store(true, Ordering::SeqCst);
                        Some(EventResult::consumed())
                    }
                    _ => Some(EventResult::Ignored),
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
                    self.cursive
                        .find_name::<TextView>(STDOUT_NAME)
                        .expect("Must exist")
                        .append(ch);
                }
            }
        }

        while let Some(message) = self.monitor_rx.try_iter().next() {
            match message {
                NotifyState(state) => {
                    info!("NotifyState: state={state:?}");
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
                        .disassembly(&self.symbols)
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
                    const CHUNK_SIZE: usize = 16;
                    let mut s = format!("{address_range}\n");
                    let mut addr = address_range.start() as usize;
                    for chunk in snapshot.chunks(CHUNK_SIZE) {
                        s.push_str(&format!("{addr:04X} "));
                        let mut chars = String::with_capacity(CHUNK_SIZE);
                        for b in chunk {
                            s.push_str(&format!(" {b:02X}"));
                            let c: char = *b as char;
                            if c.is_ascii() && !c.is_ascii_control() {
                                chars.push(c);
                            } else {
                                chars.push('.');
                            }
                        }
                        s.push_str(&String::from("   ").repeat(CHUNK_SIZE - chunk.len()));
                        s.push_str(&format!("  {chars}\n"));
                        addr += CHUNK_SIZE;
                    }
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
                        .display(&self.symbols)
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
}
