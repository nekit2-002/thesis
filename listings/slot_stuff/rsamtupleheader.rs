#[repr(C)]
pub struct RsAmTupleHeaderData {
    pub t_self: ItemPointerData,
    pub t_infomask2: u16,
    pub t_infomask: u16,
    pub rollptr: u64,
    pub t_hoff: u8,
    pub t_bits: __IncompleteArrayField<u8>,
}
