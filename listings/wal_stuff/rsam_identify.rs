#[pg_guard]
pub unsafe extern "C-unwind" fn rsam_identify(info: u8) -> *const i8 {
    let id = match info & !XLR_INFO_MASK {
        XLOG_RSAM_UNDO_INSERT => "UNDO_INSERT_RECORD".as_ptr().cast(),
        XLOG_RSAM_UNDO_UPDATE => "UNDO_UPDATE_RECORD".as_ptr().cast(),
        XLOG_RSAM_INSERT => "RSAM_INSERT".as_ptr().cast(),
        XLOG_RSAM_DELETE => "RSAM_DELETE".as_ptr().cast(),
        XLOG_RSAM_UPDATE => "RSAM_UPDATE".as_ptr().cast()
        _ => ereport!(
            PgLogLevel::PANIC,
            PgSqlErrorCode::INVALID_OPTION_CODE,
            "rsam_identify: unknown op code"
        ),
    }

    id
}
