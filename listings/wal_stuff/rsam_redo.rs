#[pg_guard]
pub unsafe extern "C-unwind" fn rsam_redo(record: *mut XLogReaderState) {
    let info = XLogRecGetInfo(record) & !XLR_INFO_MASK;
    match info & XLOG_RSAM_OPMASK {
        XLOG_RSAM_UNDO_INSERT => {
            rsam_xlog_insert_undo(record);
        }
        XLOG_RSAM_UNDO_UPDATE => {
            rsam_xlog_update_undo(record);
        }
        XLOG_RSAM_INSERT => {
            rsam_xlog_insert(record);
        }
        XLOG_RSAM_DELETE => {
            rsam_xlog_delete(record);
        }
        XLOG_RSAM_UPDATE => {
            rsam_xlog_update(record);
        }
        _ => ereport!(
            PgLogLevel::PANIC,
            PgSqlErrorCode::INVALID_OPTION_CODE,
            "rsam_redo: unknown op code"
        ),
    }
}
