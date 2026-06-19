unsafe extern "C" fn scan_begin(
    rel: *mut RelationData,
    snapshot: *mut SnapshotData,
    nkeys: i32,
    keys: *mut ScanKeyData,
    pscan: *mut ParallelTableScanDescData,
    flags: u32,
) -> pg_sys::TableScanDesc

unsafe extern "C" fn scan_end(desc: TableScanDesc)

unsafe extern "C" fn scan_getnextslot(
    scan: TableScanDesc,
    direction: ScanDirection::Type,
    slot: *mut TupleTableSlot,
) -> bool

unsafe extern "C" fn tuple_satisfies_snapshot(
    rel: Relation,
    slot: *mut TupleTableSlot,
    snapshot: Snapshot,
) -> bool


