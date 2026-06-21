#[pg_guard]
unsafe extern "C-unwind" fn tuple_delete(
    rel: Relation,
    tid: ItemPointer,
    cid: CommandId,
    snapshot: Snapshot,
    crosscheck: Snapshot,
    wait: bool,
    tmfd: *mut TM_FailureData,
    changing_part: bool,
) -> TM_Result::Type {
    let mut cid = cid;
    let xid = GetCurrentTransactionId();
    let mut vmbuffer = InvalidBuffer as i32;

    Assert(ItemPointerIsValid(tid));
    if IsInParallelMode() {
        ereport!(
            PgLogLevel::ERROR,
            PgSqlErrorCode::ERRCODE_INVALID_TRANSACTION_STATE,
            "cannot delete tuples during a parallel operation"
        )
    }

    let block = ItemPointerGetBlockNumber(tid);
    let buffer = ReadBuffer(rel, block);
    let page = BufferGetPage(buffer);
    if PageIsAllVisible(page) {
        visibilitymap_pin(rel, block, &raw mut vmbuffer);
    }

    LockBuffer(buffer, BUFFER_LOCK_EXCLUSIVE as i32);
    let lp = PageGetItemId(page, ItemPointerGetOffsetNumber(tid));
    Assert(ItemIdIsNormal!(lp));

    let mut tp = RsAmTupleData {
        t_len: (*lp).lp_len(),
        t_self: *tid,
        t_tableOid: RelationGetRelId!(rel),
        t_data: PageGetItem(page, lp).cast(),
    };

    let (mut result, have_tuple_lock) = loop {
        let mut have_tuple_lock = false;

        if vmbuffer == InvalidBuffer as i32 && PageIsAllVisible(page) {
            LockBuffer(buffer, BUFFER_LOCK_UNLOCK as i32);
            visibilitymap_pin(rel, block, &raw mut vmbuffer);
            LockBuffer(buffer, BUFFER_LOCK_EXCLUSIVE as i32);
        }
        let mut result = RsAmSatisfiesUpdate(&raw mut tp, cid, buffer);
        if result == TM_Invisible {
            UnlockReleaseBuffer(buffer);
            ereport!(
                PgLogLevel::ERROR,
                PgSqlErrorCode::ERRCODE_OBJECT_NOT_IN_PREREQUISITE_STATE,
                "attempted to delete invisible tuple!"
            )
        } else if result == TM_BeingModified && wait {
            let xwait = (*tp.t_data).t_xmax;
            let infomask = (*tp.t_data).t_infomask;
            if infomask & RSAM_XMAX_IS_MULTI as u16 != 0 {
            } else if !TransactionIdIsCurrentTransactionId(xwait) {
                LockBuffer(buffer, BUFFER_LOCK_UNLOCK as i32);
                aquire_tuplock(
                    rel,
                    &raw mut (tp.t_self),
                    LockTupleExclusive,
                    LockWaitBlock,
                    &raw mut have_tuple_lock,
                );

                XactLockTableWait(xwait, rel, &raw mut tp.t_self, XLTW_Delete);
                LockBuffer(buffer, BUFFER_LOCK_EXCLUSIVE as i32);

                if (vmbuffer == InvalidBuffer as i32 && PageIsAllVisible(page))
                    || xmax_infomask_changed((*tp.t_data).t_infomask, infomask)
                    || !((*tp.t_data).t_xmax == xwait)
                {
                    continue;
                }

                UpdateXmaxHintBits(tp.t_data, buffer, xwait);
            }

            result = if (*tp.t_data).t_infomask & RSAM_XMAX_INVALID as u16 != 0
                || xmax_is_locked_only((*tp.t_data).t_infomask)
                || RsAmTupleHeaderIsOnlyLocked(tp.t_data)
            {
                TM_Ok
            } else {
                TM_Deleted
            }
        }

        break (result, have_tuple_lock);
    };

    if result != TM_Ok {
        Assert(
            result == TM_SelfModified
                || result == TM_Updated
                || result == TM_Deleted
                || result == TM_BeingModified,
        );
        Assert((*tp.t_data).t_infomask & RSAM_XMAX_INVALID as u16 == 0);
        Assert(
            result != TM_Updated
                || !ItemPointerEquals(&raw mut (tp.t_self), &raw mut (*tp.t_data).t_ctid),
        );
    }

    if !crosscheck.is_null() && result == TM_Ok {
        if !rsam_find_appropriate_tuple(
            rel,
            buffer,
            ItemPointerGetOffsetNumber(tid),
            crosscheck,
            &raw mut tid,
            &raw mut tp,
        ) {
            result = TM_Updated;
        }
    }

    if result != TM_Ok {
        (*tmfd).xmax = tuple_header_get_update_xid(tp.t_data);
        UnlockReleaseBuffer(buffer);
        if have_tuple_lock {
            UnlockTuple(rel, &raw mut (tp.t_self), LockTupleExclusive as i32);
        }

        if vmbuffer != InvalidBuffer as i32 {
            ReleaseBuffer(vmbuffer);
        }
        return result;
    }

    MultiXactIdSetOldestMember();
    let mut new_xmax = InvalidTransactionId;
    let mut new_infomask = 0;
    let mut new_infomask2 = 0;
    compute_new_xmax_infomask(
        (*tp.t_data).t_xmax,
        (*tp.t_data).t_infomask,
        (*tp.t_data).t_infomask2,
        xid,
        LockTupleExclusive,
        true,
        &raw mut new_xmax,
        &raw mut new_infomask,
        &raw mut new_infomask2,
    );

    START_CRIT_SECTION!();

    PageSetPrunable(page, xid);
    let mut all_visible_cleared = false;
    if PageIsAllVisible(page) {
        all_visible_cleared = true;
        PageClearAllVisible(page);
        visibilitymap_clear(
            rel,
            BufferGetBlockNumber(buffer),
            vmbuffer,
            VISIBILITYMAP_VALID_BITS as u8,
        );
    }

    (*tp.t_data).t_infomask &= !(RSAM_XMAX_BITS | RSAM_MOVED) as u16;
    (*tp.t_data).t_infomask2 &= !RSAM_KEYS_UPDATED as u16;
    (*tp.t_data).t_infomask |= new_infomask;
    (*tp.t_data).t_infomask2 |= new_infomask2;
    (*tp.t_data).t_xmax = new_xmax;

    if changing_part {
        ItemPointerIndicatesMovedPartitions(&raw const (*tp.t_data).t_ctid);
    }

    MarkBufferDirty(buffer);
    if RelationNeedsWal!(rel) {
        xlrec = construct_rsam_delete_rec(&raw mut tp);
        XLogBeginInsert();
        XLogRegisterData(&raw const xlrec, std::mem::sizeof<Xl_delete_rec>());
        XLogRegisterBuffer(0, buffer, REGBUF_STANDARD);
        XLogSetRecordFlags(XLOG_INCLUDE_ORIGIN);
        let recptr = XLogInsert(RM_RSAM_ID);
        PageSetLSN(page, recptr);
    }

    END_CRIT_SECTION!();

    LockBuffer(buffer, BUFFER_LOCK_UNLOCK as i32);
    if vmbuffer != InvalidBuffer as i32 {
        ReleaseBuffer(vmbuffer);
    }

    ReleaseBuffer(buffer);
    if have_tuple_lock {
        UnlockTuple(rel, &raw mut (tp.t_self), LockTupleExclusive as i32);
    }

    pgstat_count_heap_delete(rel);

    TM_Ok
}
