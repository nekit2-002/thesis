unsafe fn rsam_xlog_insert(record: *mut XLogReaderState) {
    let lsn = (*record).EndRecPtr;
    let xlrec = XLogRecGetData(record) as *mut Xl_rsam_insert;
    let mut xlhdr = Xl_rsam_header {
        t_infomask2: 0,
        t_infomask: 0,
        rollptr: 0,
        t_hoff: 0,
    };
    let mut hdr = RsAmTupleHeaderData {
        t_xmax: InvalidTransactionId,
        t_infomask2: 0,
        t_infomask: 0,
        rollptr: 0,
        t_hoff: 0,
        t_bits: std::ptr::null_mut(),
    };
    let mut buffer = 0;
    let mut page = std::ptr::null_mut();
    let mut freespace = 0;
    let mut target_locator = RelFileLocator {
        spcOid: 0,
        dbOid: 0,
        relNumber: 0,
    };
    let mut tartget_tid = ItemPointerData {
        ip_blkid: BlockIdData { bi_hi: 0, bi_lo: 0 },
        ip_posid: 0,
    };
    let mut blkno = 0;
    XLogRecGetGetBlockTag(
        record,
        0,
        &raw mut target_locator,
        std::ptr::null_mut(),
        &raw mut blkno,
    );

    ItemPointerSetBlockNumber(&raw mut target_tid, blkno);
    ItemPointerSetOffsetNumber(&raw mut target_tid, (*xlrec).offnum);
    let action = if (XLogRecGetInfo(record) & XLOG_RSAM_INIT_PAGE) {
        buffer = XLogInitBufferForRedo(record, 0);
        page = BufferGetPage(buffer);
        PageInit(page, BufferGetPageSize(buffer), 0);
        BLK_NEEDS_REDO
    } else {
        XLogReadBufferForRedo(record, 0, &raw mut buffer)
    };
    if action == BLK_NEEDS_REDO {
        let mut datalen = 0;
        page = BufferGetPage(buffer);
        let mut data = XLogRecGetBlockData(record, 0, &raw mut datalen);
        let mut newlen = datalen - SizeOfRsAmHeader;
        memcpy(&raw mut xlhdr, data, SizeOfRsAmHeader);
        data += SizeOfRsAmHeader;
        let mut rsamtup = &raw mut hdr;
        memcpy(rsamtup as *const i8 + SizeOfRsAmTupleHeader, data, newlen);
        newlen += SizeOfRsAmTupleHeader;
        (*rsamtup).t_infomask = xlhdr.t_infomask;
        (*rsamtup).t_infomask2 = xlhdr.t_infomask2;
        (*rsamtup).t_hoff = xlhdr.t_hoff;
        (*rsamtup).rollptr = xlhdr.rollptr;
        PageAddItem(page, rsamtup, newlen, (*xlrec).offnum, true, true);
        freespace = PageGetRsAmFreeSpace(page);
        if (xlrec->flags & XLH_INSERT_ALL_VISIBLE_CLEARED){
            PageClearAllVisible(page);
        }

        PageSetLSN(page, lsn);
        MarkBufferDirty(buffer);
    }

    if BufferIsValid(buffer) {
        UnlockReleaseBuffer(buffer);
    }

    if action == BLK_NEEDS_REDO && freespace < BLCKSZ / 5 {
        XLogRecordPageWithFreeSpace(target_locator, blkno, freespace);
    }
}
