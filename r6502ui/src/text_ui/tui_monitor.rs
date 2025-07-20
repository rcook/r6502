use r6502core::TotalCycles;
use r6502cpu::Reg;
use r6502lib::emulator::{InstructionInfo, Monitor};
use r6502lib::messages::MonitorMessage;
use std::sync::mpsc::Sender;

pub struct TuiMonitor {
    monitor_tx: Sender<MonitorMessage>,
}

impl TuiMonitor {
    #[must_use]
    pub const fn new(monitor_tx: Sender<MonitorMessage>) -> Self {
        Self { monitor_tx }
    }
}

impl Monitor for TuiMonitor {
    fn on_before_execute(
        &self,
        total_cycles: TotalCycles,
        reg: Reg,
        instruction_info: InstructionInfo,
    ) {
        self.monitor_tx
            .send(MonitorMessage::BeforeExecute {
                total_cycles,
                reg,
                instruction_info,
            })
            .expect("Must succeed");
    }

    fn on_after_execute(
        &self,
        total_cycles: TotalCycles,
        reg: Reg,
        instruction_info: InstructionInfo,
    ) {
        self.monitor_tx
            .send(MonitorMessage::AfterExecute {
                total_cycles,
                reg,
                instruction_info,
            })
            .expect("Must succeed");
    }
}
