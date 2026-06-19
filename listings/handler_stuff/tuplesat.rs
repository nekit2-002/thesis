unsafe extern "C-unwind" fn tuple_satisfies_snapshot(
    rel: *mut RelationData,
    slot: *mut TupleTableSlot,
    snapshot: *mut SnapshotData,
) -> bool {
    if rel.is_null() || slot.is_null() || snapshot.is_null() {
        return false;
    }

    let rsamslot = slot as *mut RsAmTupleTableSlot;
    let mut tid = &raw mut ((*(*rsamslot).tuple).t_self);
    let tup = std::ptr::null_mut();
    let buffer = ReadBuffer(rel, ItemPointerGetBlockNumber(tid));
    LockBuffer(buffer, BUFFER_LOCK_SHARE);
    let mut res = rsam_find_appropriate_tuple(
        rel,
        buffer,
        ItemPointerGetOffsetNumber(tid),
        snapshot,
        std::ptr::null_mut(),
        &raw mut tup,
    );

    LockBuffer(buffer, BUFFER_LOCK_UNLOCK);
    ReleaseBuffer(buffer);
    if tup.is_null() {
        res = false;
    } else if (*tup).t_len != (*(*rsamslot).tuple).t_len {
        res = false;
    } else if core::slice::cmp::memcmp((*tup).t_data, (*(*rsamslot).tuple).t_data, (*tup).t_len) {
        res = false;
    }

    if !tup.is_null() {
        pfree(tup.cast());
    }
    res
}
