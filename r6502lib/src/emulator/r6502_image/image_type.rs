use num_derive::FromPrimitive;

const TYPE0: u8 = 0b00000000;
const SNAPSHOT: u8 = 0b00000001;
const RLE: u8 = 0b10000000; // TBD: run-length encoding

#[derive(Debug, FromPrimitive)]
#[repr(u8)]
pub enum R6502ImageType {
    Type0 = TYPE0,
    Type0Rle = TYPE0 | RLE, // Not implemented yet!
    Snapshot = SNAPSHOT,
    SnapshotRle = SNAPSHOT | RLE, // Not implemented yet!
}
