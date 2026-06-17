#[pg_guard]
unsafe extern "C-unwind" fn tuple_fetch_row_version(
    rel: Relation,
    tid: ItemPointer,
    snapshot: Snapshot,
    slot: *mut TupleTableSlot,
) -> bool {
    let bslot = slot as *mut BufferHeapTupleTableSlot;
    let mut buffer = InvalidBuffer as i32;
    Assert((*slot).tts_ops == &TTSOpsBufferHeapTuple);

    (*bslot).base.tupdata.t_self = *tid;

    if fetch_tuple(
        rel,
        snapshot,
        &raw mut (*bslot).base.tupdata,
        &raw mut buffer,
        false,
    ) {
        ExecStorePinnedBufferHeapTuple(&raw mut (*bslot).base.tupdata, slot, buffer);
        (*slot).tts_tableOid = RelationGetRelId!(rel);
        return true;
    }
    false
}
