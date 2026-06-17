#[pg_guard]
unsafe extern "C-unwind" fn index_fetch_tuple(
    scan: *mut IndexFetchTableData,
    tid: ItemPointer,
    snapshot: Snapshot,
    slot: *mut TupleTableSlot,
    call_again: *mut bool,
    all_dead: *mut bool,
) -> bool {
    let hscan: *mut IndexFetchHeapData = scan.cast();
    let bslot: *mut BufferHeapTupleTableSlot = slot.cast();
    
    (*hscan).xs_cbuf = ReleaseAndReadBuffer(
        (*hscan).xs_cbuf,
        (*hscan).xs_base.rel,
        ItemPointerGetBlockNumber(tid),
    );

      

    LockBuffer((*hscan).xs_cbuf, BUFFER_LOCK_SHARE as i32);
    let got_tuple = search_buffer(
        tid,
        (*hscan).xs_base.rel,
        (*hscan).xs_cbuf,
        snapshot,
        &raw mut (*bslot).base.tupdata,
        all_dead,
        !*call_again,
    );

    (*bslot).base.tupdata.t_self = *tid;
    LockBuffer((*hscan).xs_cbuf, BUFFER_LOCK_UNLOCK as i32);
    *call_again = if got_tuple {
        (*slot).tts_tableOid = RelationGetRelId!((*scan).rel);
        ExecStoreBufferHeapTuple(
            &raw mut (*bslot).base.tupdata,
            slot,
            (*hscan).xs_cbuf);
        !IsMVCCSnapshot!(snapshot)
    } else {
        false
    };

    got_tuple
}