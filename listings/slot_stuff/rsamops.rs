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
    rsamslot: *mut RsAmTupleTableSlot = slot as *mut RsAmTupleTableSlot;
    slot_deform_rsam_tuple(slot, (*rsamslot).tuple, &mut (*rsamslot).off as i32, natts)
}
