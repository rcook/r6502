use crate::{DebugMessage, Status as _Status, StatusMessage, SymbolInfo};
use anyhow::Result;
use cursive::align::HAlign;
use cursive::direction::Orientation;
use cursive::event::Key;
use cursive::view::{Nameable, Resizable, ScrollStrategy, Scrollable};
use cursive::views::{EditView, LinearLayout, Panel, TextView};
use cursive::{Cursive, CursiveRunnable, CursiveRunner};
use std::sync::mpsc::{Receiver, Sender};

const CURRENT_NAME: &str = "current";
const DISASSEMBLY_NAME: &str = "disassembly";
const STATUS_NAME: &str = "status";
const STDOUT_NAME: &str = "stdout";
const REGISTERS_NAME: &str = "registers";
const CYCLES_NAME: &str = "cycles";
const COMMAND_NAME: &str = "command";
const COMMAND_RESPONSE_NAME: &str = "command-response";

pub(crate) struct UI {
    cursive: CursiveRunner<CursiveRunnable>,
    status_rx: Receiver<StatusMessage>,
    symbols: Vec<SymbolInfo>,
}

impl UI {
    pub(crate) fn new(
        status_rx: Receiver<StatusMessage>,
        debug_tx: Sender<DebugMessage>,
        symbols: Vec<SymbolInfo>,
    ) -> Result<Self> {
        cursive::logger::init();
        let cursive = Self::make_ui(&symbols, debug_tx);
        Ok(Self {
            cursive,
            status_rx,
            symbols,
        })
    }

    pub(crate) fn run(&mut self) {
        while self.step() {}
    }

    fn make_ui(
        symbols: &[SymbolInfo],
        debug_tx: Sender<DebugMessage>,
    ) -> CursiveRunner<CursiveRunnable> {
        use crate::DebugMessage::*;

        fn panel<V>(view: V, label: &str) -> Panel<V> {
            Panel::new(view).title(label).title_position(HAlign::Left)
        }

        let s = symbols
            .iter()
            .map(|s| format!("{} = ${:04X}", s.name, s.value))
            .collect::<Vec<_>>()
            .join("\n");

        let status = TextView::new("")
            .with_name(STATUS_NAME)
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
            TextView::new("Q: Quit\nSpace: Step\nR: Run\nB: Break\nC: Command\nEsc: Exit command")
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
                    // TBD: For now we'll implement a single command "m BEGIN END"
                    // Let's fetch the stack for the time being
                    let parts = text.split_whitespace().collect::<Vec<_>>();
                    if parts.len() != 3 {
                        return;
                    }

                    if parts[0] != "m" {
                        return;
                    }

                    let begin = u16::from_str_radix(parts[1], 16).expect("Must succeed");
                    let end = u16::from_str_radix(parts[2], 16).expect("Must succeed");

                    _ = d.send(FetchMemory { begin, end });
                    s.call_on_name(COMMAND_NAME, |command: &mut EditView| {
                        command.disable();
                    });
                }
                nasty_hack(s, text, &d)
            })
            .with_name(COMMAND_NAME)
            .fixed_height(1);

        let left = LinearLayout::new(Orientation::Vertical)
            .child(panel(current, "Current Instruction"))
            .child(panel(registers, "Registers"))
            .child(panel(cycles, "Cycles"))
            .child(panel(disassembly, "Disassembly"))
            .child(panel(status, "Status"));

        let right = LinearLayout::new(Orientation::Vertical)
            .child(panel(stdout, "Output"))
            .child(panel(symbols, "Symbols"))
            .child(panel(help, "Help"))
            .child(panel(command_response, "Command Response"))
            .child(panel(command, "Command"));

        let dashboard = LinearLayout::new(Orientation::Horizontal)
            .child(left)
            .child(right);

        let mut cursive = cursive::default().into_runner();
        cursive.add_fullscreen_layer(dashboard);

        cursive.set_fps(30);

        cursive.add_global_callback('~', Cursive::toggle_debug_console);
        cursive.add_global_callback('q', Cursive::quit);
        let d = debug_tx.clone();
        cursive.add_global_callback(' ', move |_| _ = d.send(Step));
        let d = debug_tx.clone();
        cursive.add_global_callback('r', move |_| _ = d.send(Run));
        let d = debug_tx.clone();
        cursive.add_global_callback('b', move |_| _ = d.send(Break));
        cursive.add_global_callback('c', move |s| {
            s.call_on_name(COMMAND_NAME, |command: &mut EditView| {
                // TBD: Figure out how to give the edit view focus
                command.enable();
            });
        });
        cursive.add_global_callback(Key::Esc, move |s| {
            s.call_on_name(COMMAND_NAME, |command: &mut EditView| {
                command.disable();
            });
        });

        cursive
    }

    fn step(&mut self) -> bool {
        use crate::StatusMessage::*;

        if !self.cursive.is_running() {
            return false;
        }

        while let Some(message) = self.status_rx.try_iter().next() {
            match message {
                BeforeExecute {
                    reg,
                    cycles,
                    instruction,
                } => {
                    self.cursive
                        .find_name::<TextView>(REGISTERS_NAME)
                        .expect("Must exist")
                        .set_content(reg.pretty());
                    self.cursive
                        .find_name::<TextView>(CYCLES_NAME)
                        .expect("Must exist")
                        .set_content(format!("{cycles}"));
                    self.cursive
                        .find_name::<TextView>(CURRENT_NAME)
                        .expect("Must exist")
                        .set_content(instruction.pretty_current(&self.symbols));
                }
                AfterExecute {
                    reg,
                    cycles,
                    instruction,
                } => {
                    self.cursive
                        .find_name::<TextView>(REGISTERS_NAME)
                        .expect("Must exist")
                        .set_content(reg.pretty());
                    self.cursive
                        .find_name::<TextView>(CYCLES_NAME)
                        .expect("Must exist")
                        .set_content(format!("{cycles}"));

                    let mut s = instruction.pretty_disassembly(&self.symbols);
                    s.push('\n');
                    self.cursive
                        .find_name::<TextView>(DISASSEMBLY_NAME)
                        .expect("Must exist")
                        .append(s);
                }
                Status(status) => {
                    let mut s = match status {
                        _Status::Halted => String::from("Halted"),
                    };
                    s.push('\n');
                    self.cursive
                        .find_name::<TextView>(STATUS_NAME)
                        .expect("Must exist")
                        .append(s);
                }
                WriteStdout(c) => {
                    self.cursive
                        .find_name::<TextView>(STDOUT_NAME)
                        .expect("Must exist")
                        .append(c);
                }
                FetchMemoryResponse {
                    begin,
                    end,
                    snapshot,
                } => {
                    const CHUNK_SIZE: usize = 16;
                    let mut s = format!("${begin:04X}:${end:04X}\n");
                    let mut addr = begin as usize;
                    for chunk in snapshot.chunks(CHUNK_SIZE) {
                        s.push_str(&format!("{addr:04X} "));
                        let mut chars = String::with_capacity(CHUNK_SIZE);
                        for b in chunk {
                            s.push_str(&format!(" {b:02X}"));
                            let c: char = *b as char;
                            if c.is_ascii() && !c.is_ascii_control() {
                                chars.push(c)
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
                        .append(s)
                }
            }
        }

        self.cursive.step();
        true
    }
}
