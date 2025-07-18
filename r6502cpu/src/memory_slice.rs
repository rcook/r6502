pub struct MemorySlice<'a> {
    pub bytes: &'a [u8],
    pub load: u16,
}
