#[pg_guard]
unsafe extern "C-unwind" fn search_buffer(
    tid: ItemPointer,
    rel: *mut RelationData,
    buffer: Buffer,
    snapshot: *mut SnapshotData,
    heap_tuple: *mut HeapTupleData,
    all_dead: *mut bool,
    fst_call: bool,
) -> bool {
    let page = BufferGetPage(buffer);
    let mut prev_xmax = InvalidTransactionId;
    let mut vistest: *mut GlobalVisState = std::ptr::null_mut();

    if !all_dead.is_null() {
        *all_dead = fst_call;
    }

    let (block_n, mut offnum) = (
        ItemPointerGetBlockNumber(tid),
        ItemPointerGetOffsetNumber(tid),
    );

    let mut at_chain_start = fst_call;
    let mut skip = !fst_call;
    loop {
        if offnum < FirstOffsetNumber || offnum > PageGetMaxOffsetNumber(page) {
            break;
        }

        let lp = PageGetItemId(page, offnum);
        if !ItemIdIsNormal!(lp) {
            if (*lp).lp_flags() == LP_REDIRECT && at_chain_start {
                offnum = (*lp).lp_off() as u16;
                at_chain_start = false;
                continue;
            }
            break;
        }

        (*heap_tuple).t_data = PageGetItem(page, lp).cast();
        (*heap_tuple).t_len = (*lp).lp_len();
        (*heap_tuple).t_tableOid = RelationGetRelId!(rel);
        ItemPointerSet(&raw mut (*heap_tuple).t_self, block_n, offnum);

        if at_chain_start && HeapTupleHeaderIsHeapOnly((*heap_tuple).t_data) {
            break;
        }

        if prev_xmax != 0.into() && prev_xmax != HeapTupleHeaderGetXmin((*heap_tuple).t_data) {
            break;
        }

        if !skip {
            let valid = tuple_satisfies_visibility(heap_tuple, snapshot, buffer);
            if valid {
                ItemPointerSetOffsetNumber(tid, offnum);
                PredicateLockTID(
                    rel,
                    &raw mut (*heap_tuple).t_self,
                    snapshot,
                    HeapTupleHeaderGetXmin((*heap_tuple).t_data),
                );
                if !all_dead.is_null() {
                    *all_dead = false;
                }
                return true;
            }
        }

        skip = false;
        if !all_dead.is_null() && *all_dead {
            if vistest.is_null() {
                vistest = GlobalVisTestFor(rel);
            }

            if !HeapTupleIsSurelyDead(heap_tuple, vistest) {
                *all_dead = false;
            }
        }

        if HeapTupleHeaderIsHotUpdated((*heap_tuple).t_data) {
            offnum = ItemPointerGetOffsetNumber(&raw const (*(*heap_tuple).t_data).t_ctid);
            at_chain_start = false;
            prev_xmax = tuple_header_get_update_xid((*heap_tuple).t_data);
        } else {
            break;
        }
    }
    false
}