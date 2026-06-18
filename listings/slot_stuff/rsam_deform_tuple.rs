#[pg_guard]
unsafe extern "C-unwind" fn slot_deform_rsam_tuple(
    slot: *mut TupleTableSlot,
    tuple: RsAmTuple,
    offp: *mut u32,
    natts: i32,
) {
}
