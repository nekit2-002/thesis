#[pg_guard]
pub unsafe extern "C-unwind" fn fetch_tuple(
    rel: Relation,
    snapshot: Snapshot,
    tup: *mut RsAmTuple,
    user_buf: *mut Buffer,
    keep_buf: bool,
) -> bool {
    let tid = &raw mut (*tup).t_self;
    let mut ctup = tup;
    let buffer = ReadBuffer(rel, ItemPointerGetBlockNumber(tid));

    LockBuffer(buffer, BUFFER_LOCK_SHARE as i32);

    let page = BufferGetPage(buffer);
    let offnum = ItemPointerGetOffsetNumber(tid);

    if offnum < FirstOffsetNumber || offnum > PageGetMaxOffsetNumber(page) {
        LockBuffer(buffer, BUFFER_LOCK_UNLOCK as i32);
        if keep_buf {
            *user_buf = buffer;
        } else {
            ReleaseBuffer(buffer);
            *user_buf = InvalidBuffer as i32;
        }
        (*tup).t_data = std::ptr::null_mut();
        return false;
    }

    let lp = PageGetItemId(page, offnum);
    if !ItemIdIsNormal!(lp) {
        LockBuffer(buffer, BUFFER_LOCK_UNLOCK as i32);
        ReleaseBuffer(buffer);
        *user_buf = InvalidBuffer as i32;
        (*tup).t_data = std::ptr::null_mut();
        return false;
    }

    (*tup).t_data = PageGetItem(page, lp).cast();
    (*tup).t_len = (*lp).lp_len();
    (*tup).t_tableOid = RelationGetRelId!(rel);
    let mut resulttup = std::ptr::null_mut();

    let valid =
        rsam_find_appropriate_tuple(relation, buffer, offnum, snapshot, tid, &raw mut resulttup);
    LockBuffer(buffer, BUFFER_LOCK_UNLOCK as i32);
    if valid && !resulttup.is_null() {
        *tup = resulttup;
        *user_buf = buffer;
        return true;
    }

    *user_buf = if keep_buf {
        buffer
    } else {
        ReleaseBuffer(buffer);
        (*tup).t_data = std::ptr::null_mut();
        InvalidBuffer as i32
    };
    false
}
