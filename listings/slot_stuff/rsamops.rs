pub static TTSOpsRsAmTuple: TupleTableSlotOps = TupleTableSlotOps {
    base_slot_size: std::mem::size_of::<RsAmTupleTableSlot>(),
    init: Some(tts_rsam_init),
    release: Some(tts_rsam_release),
    clear: Some(tts_rsam_clear),
    getsomeattrs: Some(tts_rsam_getsomeattrs),
    getsysattr: Some(tts_rsam_getsysattr),
    is_current_xact_tuple: Some(tts_rsam_is_current_xact_tuple),
    materialize: Some(tts_rsam_materialize),
    copyslot: Some(tts_rsam_copyslot),

    get_heap_tuple: None,
    get_minimal_tuple: None,

    copy_heap_tuple: Some(tts_rsam_copy_heap_tuple),
    copy_minimal_tuple: Some(tts_rsam_copy_minimal_tuple),
};

#[pg_guard]
unsafe extern "C-unwind" fn tts_rsam_init(_slot: *mut TupleTableSlot) {}

#[pg_guard]
unsafe extern "C-unwind" fn tts_rsam_release(_slot: *mut TupleTableSlot) {}

#[pg_guard]
unsafe extern "C-unwind" fn tts_rsam_getsomeattrs(slot: *mut TupleTableSlot, natts: i32) {
    let rsamslot = slot as *mut RsAmTupleTableSlot;
    slot_deform_rsam_tuple(
        slot,
        (*rsamslot).tuple,
        &raw mut (*rsamslot).off as i32,
        natts,
    )
}

#[pg_guard]
unsafe extern "C-unwind" fn tts_rsam_getsysattr(
    slot: *mut TupleTableSlot,
    attnum: i32,
    isnull: *mut bool,
) -> Datum {
    let rsamslot = slot as *mut RsAmTupleTableSlot;
    rsam_getsysattr(
        (*rsamslot).tuple,
        attnum,
        (*slot).tts_TupleDescriptor,
        isnull,
    )
}

#[pg_guard]
unsafe extern "C-unwind" fn tts_rsam_clear(slot: *mut TupleTableSlot) {
    let rsamslot = slot as *mut RsAmTupleTableSlot;
    if TTS_SHOULDFREE(slot) {
        pfree((*rsamslot).tuple.cast());
        (*slot).tts_flags &= !TTS_FLAG_SHOULDFREE;
    }

    (*slot).tts_nvalid = 0;
    (*slot).tts_flags |= TTS_FLAG_EMPTY;
    (*rsamslot).tuple = std::ptr::null_mut();
    (*rsamslot).off = 0;
}

#[pg_guard]
unsafe extern "C-unwind" fn tts_rsam_copy_heap_tuple(slot: *mut TupleTableSlot) -> HeapTuple {
    let tuple = heap_form_tuple(
        (*slot).tts_TupleDescriptor,
        (*slot).tts_values,
        (*slot).tts_isnull,
    );
    (*tuple).t_self = (*slot).tts_tid;
    (*tuple).t_tableOid = (*slot).tts_tableOid;
    tuple
}
#[pg_guard]
unsafe extern "C-unwind" fn tts_rsam_copy_minimal_tuple(slot: *mut TupleTableSlot) -> MinimalTuple {
    heap_form_minimal_tuple(
        (*slot).tts_TupleDescriptor,
        (*slot).tts_values,
        (*slot).tts_isnull,
    )
}

#[pg_guard]
unsafe extern "C-unwind" fn tts_rsam_materialize(slot: *mut TupleTableSlot) {
    let mut rsamslot = slot as *mut RsAmTupleTableSlot;
    if TTS_SHOULDFREE(slot) {
        return;
    }

    (*slot).tts_flags |= TTS_FLAG_SHOULDFREE;

    let oldContext = MemoryContextSwitchTo((*slot).tts_mcxt);
    (*rsamslot).tuple = if (*rsamslot).tuple.is_null() {
        rsam_copytuple((*rsamslot).tuple)
    } else {
        rsam_form_tuple(
            (*slot).tts_TupleDescriptor,
            (*slot).tts_values,
            (*slot).tts_isnull,
        )
    };

    MemoryContextSwitchTo(oldContext);
    (*slot).tts_nvalid = 0;
    (*rsamslot).off = 0;
}

#[pg_guard]
unsafe extern "C-unwind" fn tts_rsam_copyslot(
    dstslot: *mut TupleTableSlot,
    srcslot: *mut TupleTableSlot,
) {
    let oldContext = MemoryContextSwitchTo((*dstslot).tts_mcxt);
    let mut tuple = ExecCopySlotHeapTuple(srcslot);
    MemoryContextSwitchTo(oldContext);
    ExecForceStoreHeapTuple(tuple, dstslot, false);
    ExecMatrializeSlot(dstslot);
    pfree(tuple.cast())
}

#[pg_guard]
unsafe extern "C-unwind" fn tts_rsam_is_current_xact_tuple(slot: *mut TupleTableSlot) {
    let rsamslot = slot as *mut RsAmTupleTableSlot;
    let xmin = RsAmTupleGetXMinFromUndo((*rsamslot).tuple);
    TransactionIdIsCurrentTransactionId(xmin)
}
