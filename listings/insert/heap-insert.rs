#[pg_guard]
pub unsafe extern "C-unwind" fn rsam_insert(
    rel: *mut RelationData,
    tup: *mut RsAmTupleData,
    cid: CommandId,
    options: i32,
    state: *mut BulkInsertStateData,
) {
    let xid = GetCurrentTransactionId();
    let mut vmbuffer = InvalidBuffer as i32;
    let mut all_visible_cleared = false;
    Assert(RsAmTupleHeaderGetNatts((*tup).t_data) <= (*(*rel).rd_rel).relnatts as u16);

    let tuple = prepare_insert(rel, tup, xid, cid, options);
    let buffer = RelationGetBufferForRsAmTuple(
        rel,
        (*tuple).t_len as usize,
        InvalidBuffer as i32,
        options,
        state,
        &raw mut vmbuffer,
        std::ptr::null_mut(),
        0,
    );

    START_CRIT_SECTION!();
    let insert_record = contstruct_undo_record(tup, rel, cid, options);
    let latest_undo_ptr = GetCurrentUndoPtr();
    let undo_buffer = GetUndoBufferByPtr(latest_undo_ptr);
    let undo_page = BufferGetPage(undo_buffer);
    insert_undo_record(insert_record, latest_undo_ptr);
    if RelationNeedsWal!(relation) {
        let xl_undo_insert_rec = construct_undo_insert_wal_record(insert_record, undo_page);
        let xl_undo_insert_header = extract_header_from_undo_record(insert_record);
        XLogBeginInsert();
        XLogRegisterData(&raw const xl_undo_insert_rec, std::mem::sizeof<Xl_undo_insert>());
        XLogRegisterBuffer(0, REGBUF_STANDARD)
        XLogRegisterBufData(0, &raw mut xl_undo_insert_header, SizeOfUndoInsertRecordHeader);
        XLogRegisterBufData(
            0,
            &raw const xl_undo_insert_rec.data as *mut i8 + SizeOfUndoInsertRecordHeader,
            xl_undo_insert_rec.rec_len - SizeOfUndoInsertRecordHeader
        );
        XLogSetRecordFlags(XLOG_INCLUDE_ORIGIN);
        let recptr = XLogInsert(RM_RSAM_ID);
        PageSetLSN(undo_page, recptr);
    }


    relation_put_tuple(rel, buffer, tuple);
    if PageIsAllVisible(BufferGetPage(buffer)) {
        all_visible_cleared = true;
        PageClearAllVisible(BufferGetPage(buffer));
        visibilitymap_clear(
            rel,
            ItemPointerGetBlockNumber(&raw const (*tuple).t_self),
            vmbuffer,
            VISIBILITYMAP_VALID_BITS as u8,
        );
    }

    MarkBufferDirty(buffer);
    if RelationNeedsWal!(relation) {
        let xl_insert_rec = construct_rsam_insert_rec(tup);
        XLogBeginInsert();
        XLogRegisterData(&raw const xl_insert_rec, std::mem::sizeof<Xl_rsam_insert>());

        XLogRegisterBuffer(0, buffer, REGBUF_STANDARD);
        XLogRegisterBufData(0,
            (*tup).t_data as *mut i8 + SizeOfRsAmTupleHeader,
            (*tup).t_len - SizeOfRsAmTupleHeader,
        );

        XLogSetRecordFlags(XLOG_INCLUDE_ORIGIN);
        let recptr = XLogInsert(RM_RSAM_ID);
        PageSetLSN(BufferGetPage(buffer), recptr);
    }
    END_CRIT_SECTION!();

    UnlockReleaseBuffer(buffer);
    if vmbuffer != InvalidBuffer as i32 {
        ReleaseBuffer(vmbuffer);
    }

    pgstat_count_heap_insert(rel, 1);
    if tuple != tup {
        (*tup).t_self = (*tuple).t_self;
        pfree(tuple.cast());
    }
}
