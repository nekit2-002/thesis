#[pg_guard]
unsafe extern "C-unwind" fn scan_begin(
    rel: *mut RelationData,
    snapshot: *mut SnapshotData,
    nkeys: i32,
    key: ScanKey,
    pscan: *mut ParallelTableScanDescData,
    flags: u32,
) -> TableScanDesc {
    RelationIncrementReferenceCount(rel);

    let scan = palloc(std::mem::size_of::<HeapScanDescData>()) as HeapScanDesc;
    (*scan).rs_base.rs_rd = rel;
    (*scan).rs_base.rs_snapshot = snapshot;
    (*scan).rs_base.rs_nkeys = nkeys;
    (*scan).rs_base.rs_flags = flags;
    (*scan).rs_base.rs_parallel = pscan;
    (*scan).rs_strategy = std::ptr::null_mut();
    (*scan).rs_cbuf = InvalidBuffer as i32;
    (*scan).rs_ctup.t_tableOid = (*rel).rd_id;

    if snapshot.is_null() || !IsMVCCSnapshot!(snapshot) {
        (*scan).rs_base.rs_flags &= !SO_ALLOW_PAGEMODE;
    }

    (*scan).rs_parallelworkerdata = if !pscan.is_null() {
        palloc(std::mem::size_of::<ParallelBlockTableScanWorkerData>()).cast()
    } else {
        std::ptr::null_mut()
    };

    (*scan).rs_base.rs_key = if nkeys > 0 {
        palloc(std::mem::size_of::<ScanKeyData>()) as ScanKey
    } else {
        std::ptr::null_mut()
    };

    initscan(scan, key, false);

    (*scan).rs_read_stream = std::ptr::null_mut();

    if (*scan).rs_base.rs_flags & SO_TYPE_SEQSCAN != 0 {
        let cb: ReadStreamBlockNumberCB = Some(heap_scan_stream_read_next_serial);
        (*scan).rs_read_stream = read_stream_begin_relation(
            READ_STREAM_SEQUENTIAL as i32,
            (*scan).rs_strategy,
            (*scan).rs_base.rs_rd,
            MAIN_FORKNUM,
            cb,
            scan.cast(),
            0,
        )
    }

    scan as TableScanDesc
}