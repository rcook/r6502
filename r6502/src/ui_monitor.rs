use crate::MonitorMessage;
use r6502lib::{InstructionInfo, Monitor, Reg, TotalCycles};
use std::sync::mpsc::Sender;

pub(crate) struct UiMonitor {
    monitor_tx: Sender<MonitorMessage>,
}

impl UiMonitor {
    pub(crate) const fn new(monitor_tx: Sender<MonitorMessage>) -> Self {
        Self { monitor_tx }
    }
}

impl Monitor for UiMonitor {
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
