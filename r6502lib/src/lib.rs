#![warn(clippy::missing_const_for_fn)]
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

pub mod deserialization;
pub mod util;

mod address_range;
mod addressing_mode;
mod binding;
mod bus;
mod bus_device;
mod bus_event;
mod bus_view;
mod byte_op;
mod constants;
mod cpu;
mod device_mapping;
mod frequency;
mod image;
mod image_format;
mod image_header;
mod image_slice;
mod instruction;
mod instruction_info;
mod instruction_set;
mod machine_tag;
mod machine_type;
mod monitor;
mod no_operand_op;
mod op;
mod op_cycles;
mod op_info;
mod opcode;
mod operand;
mod ops;
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
pub use bus_device::BusDevice;
pub use bus_event::BusEvent;
pub use bus_view::BusView;
pub use byte_op::ByteOp;
pub use constants::{IRQ, IRQ_ADDR, MEMORY_SIZE, RESET, STACK_BASE};
pub use cpu::Cpu;
pub use device_mapping::DeviceMapping;
pub use image::Image;
pub use image_format::ImageFormat;
pub use image_slice::ImageSlice;
pub use instruction_info::InstructionInfo;
pub use instruction_set::{InstructionSet, MOS_6502};
pub use machine_tag::MachineTag;
pub use machine_type::MachineType;
pub use monitor::{DummyMonitor, Monitor, TracingMonitor};
pub use no_operand_op::NoOperandOp;
pub use op_info::OpInfo;
pub use opcode::Opcode;
pub use p::P;
pub use pia::Pia;
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
    DEFAULT_LOAD, DEFAULT_SP, DEFAULT_START, NMI, R6502_DUMP_MAGIC_NUMBERS, R6502_MAGIC_NUMBER,
    SIM6502_MAGIC_NUMBER,
};
#[allow(unused)]
pub(crate) use frequency::Frequency;
pub(crate) use image_header::ImageHeader;
pub(crate) use instruction::Instruction;
pub(crate) use op::Op;
pub(crate) use op_cycles::OpCycles;
pub(crate) use operand::Operand;

#[cfg(test)]
pub(crate) use p::p;
