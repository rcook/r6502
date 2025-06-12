#[allow(unused)]
pub(crate) struct ImageSlice<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) load: u16,
}
