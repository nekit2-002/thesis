#[pg_guard]
pub unsafe extern "C-unwind" fn rsam_desc(buf: *mut StringInfoData, record: *mut XLogReaderState) {
    let rec = XLogRecGetData(record);
    let mut info = XLogRecGetInfo(record) & !XLR_INFO_MASK;
    info &= XLOG_RSAM_OPMASK;
    if info == XLOG_RSAM_UNDO_INSERT {
        let xlrec = rec as *mut Xl_undo_insert_rec;
        appendStringInfo(
            buf,
            "page_offset: {}, remained len: {}",
            (*xlrec).page_offset,
            (*xlrec).rem_len,
        );
    } else if info == XLOG_RSAM_UNDO_UPDATE {
        let xlrec = rec as *mut Xl_undo_update_rec;
        appendStringInfo(
            buf,
            "page_offset: {}, remained len: {}, rollptr: {}",
            (*xlrec).page_offset,
            (*xlrec).rem_len,
            (*xlrec).rollptr,
        );
    } else if info == XLOG_RSAM_INSERT {
        let xlrec = rec as *mut Xl_rsam_insert;
        appendStringInfo(buf, "off: {}, flags: {}", (*xlrec).offnum, (*xlrec).flags)
    } else if info == XLOG_RSAM_DELETE {
        let xlrec = rec as *mut Xl_rsam_delete;
        appendStringInfo(
            buf,
            "xmax: {}, off: {}, infobits: {}, flags: {}",
            (*xlrec).xmax,
            (*xlrec).offnum,
            (*xlrec).infobits_set,
            (*xlrec).flags,
        )
    } else if info == XLOG_RSAM_UPDATE {
        let xlrec = rec as *mut Xl_rsam_update;
        appendStringInfo(
            buf,
            "new_xmax: {}, off: {}, infobits: {}, flags: {}, rollptr: {}",
            (*xlrec).new_xmax,
            (*xlrec).offnum,
            (*xlrec).infobits_set,
            (*xlrec).flags,
            (*xlrec).rollptr,
        )
    }
}
