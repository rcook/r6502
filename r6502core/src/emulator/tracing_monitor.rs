use crate::Reg;
use crate::emulator::{InstructionInfo, Monitor};
use crate::symbols::MapFile;
use log::info;
use r6502lib::TotalCycles;

#[derive(Default)]
pub struct TracingMonitor {
    map_file: MapFile,
}

impl TracingMonitor {
    #[must_use]
    pub const fn new(map_file: MapFile) -> Self {
        Self { map_file }
    }
}

impl Monitor for TracingMonitor {
    fn on_before_execute(
        &self,
        _total_cycles: TotalCycles,
        reg: Reg,
        instruction_info: InstructionInfo,
    ) {
        info!(
            "{disassembly:<50}  A={a:02X} X={x:02X} Y={y:02X} P={p} SP={sp:02X}",
            disassembly = instruction_info
                .disassembly(&self.map_file)
                .expect("Must succeed"),
            a = reg.a,
            x = reg.x,
            y = reg.y,
            p = reg.p,
            sp = reg.sp
        );
    }
}
