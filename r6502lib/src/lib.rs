#![warn(clippy::pedantic)]
#![allow(clippy::bool_to_int_with_if)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::verbose_bit_mask)]
#![allow(missing_docs)]

pub mod util;

mod address_range;
mod addressing_mode;
mod binding;
mod bus;
mod bus_event;
mod bus_view;
mod byte_op;
mod constants;
mod cpu;
mod device_mapping;
mod frequency;
mod image;
mod image_format;
mod instruction;
mod instruction_info;
mod instruction_set;
mod left_open_interval;
mod machine_type;
mod memory_mapped_device;
mod monitor;
mod no_operand_op;
mod op;
mod op_cycles;
mod op_info;
mod opcode;
mod operand;
mod ops;
mod os;
mod p;
mod pia;
mod ram;
mod reg;
mod rom;
mod scenario_runner;
mod symbol_info;
mod test_scenarios;
mod total_cycles;
mod word_op;

pub use address_range::AddressRange;
pub use bus::Bus;
pub use bus_event::BusEvent;
pub use bus_view::BusView;
pub use byte_op::ByteOp;
pub use constants::{IRQ, IRQ_ADDR, MEMORY_SIZE, OSHALT, OSWRCH, RESET, STACK_BASE};
pub use cpu::Cpu;
pub use image::Image;
pub use image_format::ImageFormat;
pub use instruction_info::InstructionInfo;
pub use instruction_set::{InstructionSet, MOS_6502};
pub use machine_type::MachineType;
pub use memory_mapped_device::MemoryMappedDevice;
pub use monitor::{DummyMonitor, Monitor, TracingMonitor};
pub use no_operand_op::NoOperandOp;
pub use op_info::OpInfo;
pub use opcode::Opcode;
pub use os::Os;
pub use p::P;
pub use ram::Ram;
pub use reg::Reg;
pub use rom::Rom;
pub use scenario_runner::{run_scenario, run_scenarios_with_filter};
pub use symbol_info::SymbolInfo;
pub use total_cycles::TotalCycles;
pub use word_op::WordOp;

pub(crate) use addressing_mode::AddressingMode;
pub(crate) use binding::Binding;
pub(crate) use constants::{
    DEFAULT_LOAD, DEFAULT_SP, DEFAULT_START, NMI, PIA_END_ADDR, PIA_START_ADDR,
    R6502_DUMP_MAGIC_NUMBERS, R6502_MAGIC_NUMBER, SIM6502_MAGIC_NUMBER,
};
pub(crate) use device_mapping::DeviceMapping;
pub(crate) use frequency::Frequency;
pub(crate) use instruction::Instruction;
#[allow(unused)]
pub(crate) use left_open_interval::LeftOpenInterval;
pub(crate) use op::Op;
pub(crate) use op_cycles::OpCycles;
pub(crate) use operand::Operand;
pub(crate) use pia::Pia;

#[cfg(test)]
pub(crate) use p::p;
