unsafe fn rsam_redo_insert(record: *mut XLogReaderState) {
    let raw = XLogRecGetData(record);

    let hdr = &*(raw as *const RsamWalInsertUndo);

    rsam_undo_write_at(
        hdr.undo_ptr,
        raw,
        std::mem::size_of::<RsamWalInsertUndo>() + hdr.pk_len as usize,
    );
}
