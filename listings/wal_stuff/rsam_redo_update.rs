unsafe fn rsam_redo_update(record: *mut XLogReaderState) {
    let raw = XLogRecGetData(record);

    let hdr = &*(raw as *const RsamWalUpdateUndo);

    let total_size = rsam_update_record_size(hdr);

    rsam_undo_write_at(hdr.undo_ptr, raw, total_size);
}
