#[pg_guard]
unsafe extern "C-unwind" fn tuple_fetch_row_version(
    rel: Relation,
    tid: ItemPointer,
    snapshot: Snapshot,
    slot: *mut TupleTableSlot,
) -> bool {
    let rsamslot = slot as *mut RsAmTupleTableSlot;
    let mut buffer = InvalidBuffer as i32;
    Assert((*slot).tts_ops == &TTSOpsRsAmTuple);
    rollback_tuple_if_transaction_aborted(tid);

    if fetch_tuple(
        rel,
        snapshot,
        &raw mut (*rsamslot).tuple,
        &raw mut buffer,
        false,
    ) {
        ExecStorePinnedRsAmTuple(&raw mut (*rsamslot).tuple, slot, buffer);
        (*slot).tts_tableOid = RelationGetRelId!(rel);
        return true;
    }
    false
}
