unsafe fn rsam_redo_apply_tuple(record: *mut XLogReaderState) {
    let mut buffer: Buffer = InvalidBuffer;

    let action = XLogReadBufferForRedo(record, 0, &mut buffer);

    if action == BLK_NOTFOUND {
        return;
    }

    let page = BufferGetPage(buffer);

    let raw = XLogRecGetData(record);

    let hdr = &*(raw as *const RsamWalApplyTuple);

    let tuple_ptr = raw.add(std::mem::size_of::<RsamWalApplyTuple>());

    rsam_page_apply_tuple(page, tuple_ptr, hdr.tuple_len, hdr.undo_ptr);

    MarkBufferDirty(buffer);

    UnlockReleaseBuffer(buffer);
}
