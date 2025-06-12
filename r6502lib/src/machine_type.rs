use crate::{
    AddressRange, DeviceDescription, Pia, Ram, Rom, MEMORY_SIZE, PIA_END_ADDR, PIA_START_ADDR,
};

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum MachineType {
    AllRam = 0,
    Custom = 10,
    Acorn = 30,
    Apple1 = 40,
}

impl MachineType {
    pub(crate) fn get_device_descriptions(self) -> Vec<DeviceDescription> {
        match self {
            Self::AllRam => vec![DeviceDescription {
                address_range: AddressRange::new(0x0000, 0xffff).expect("Must succeed"),
                device_fn: Box::new(|_, image_slice| {
                    Box::new(Ram::<MEMORY_SIZE>::new(&image_slice))
                }),
                offset: 0x0000,
            }],
            Self::Custom => vec![
                DeviceDescription {
                    address_range: AddressRange::new(0x0000, 0xfbff).expect("Must succeed"),
                    device_fn: Box::new(|_, image_slice| {
                        Box::new(Ram::<{ 0xfbff + 1 }>::new(&image_slice))
                    }),
                    offset: 0x0000,
                },
                DeviceDescription {
                    address_range: AddressRange::new(0xfc00, 0xfc03).expect("Must succeed"),
                    device_fn: Box::new(|bus_tx, _| Box::new(Pia::new(bus_tx))),
                    offset: 0xfc00,
                },
                DeviceDescription {
                    address_range: AddressRange::new(0xfc04, 0xffff).expect("Must succeed"),
                    device_fn: Box::new(|_, image_slice| {
                        Box::new(Ram::<{ 0xffff - 0xfc04 + 1 }>::new(&image_slice))
                    }),
                    offset: 0xfc04,
                },
            ],
            Self::Acorn => vec![
                DeviceDescription {
                    address_range: AddressRange::new(0x0000, 0x7fff).expect("Must succeed"),
                    device_fn: Box::new(|_, image_slice| {
                        Box::new(Ram::<0x8000>::new(&image_slice))
                    }),
                    offset: 0x0000,
                },
                DeviceDescription {
                    address_range: AddressRange::new(0x8000, 0xffff).expect("Must succeed"),
                    device_fn: Box::new(|_, image_slice| {
                        Box::new(Rom::<0x8000>::new(&image_slice))
                    }),
                    offset: 0x8000,
                },
            ],
            Self::Apple1 => vec![
                DeviceDescription {
                    address_range: AddressRange::new(0x0000, PIA_START_ADDR - 1)
                        .expect("Must succeed"),
                    device_fn: Box::new(|_, image_slice| {
                        Box::new(Ram::<{ PIA_START_ADDR as usize }>::new(&image_slice))
                    }),
                    offset: 0x0000,
                },
                DeviceDescription {
                    address_range: AddressRange::new(PIA_START_ADDR, PIA_END_ADDR)
                        .expect("Must succeed"),
                    device_fn: Box::new(|bus_tx, _| Box::new(Pia::new(bus_tx))),
                    offset: PIA_START_ADDR,
                },
                DeviceDescription {
                    address_range: AddressRange::new(PIA_END_ADDR + 1, 0xffff)
                        .expect("Must succeed"),
                    device_fn: Box::new(|_, image_slice| {
                        Box::new(Ram::<{ 0xffff - PIA_END_ADDR as usize }>::new(&image_slice))
                    }),
                    offset: PIA_END_ADDR + 1,
                },
            ],
        }
    }
}
