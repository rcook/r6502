use crate::terminal_ui::{SimpleOutput, VduDriver};
use r6502config::OutputDeviceType;
use r6502lib::emulator::OutputDevice;

#[must_use]
pub fn create_output_device(output_device_type: &OutputDeviceType) -> Box<dyn OutputDevice> {
    match output_device_type {
        OutputDeviceType::Simple => Box::new(SimpleOutput),
        OutputDeviceType::VduDriver => Box::new(VduDriver::default()),
    }
}
