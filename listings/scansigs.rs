unsafe extern "C" fn scan_begin(
    _rel: *mut RelationData, 
    _snapshot: *mut SnapshotData, 
    _nkeys: i32,
    _keys: *mut ScanKeyData,
    _pscan: *mut ParallelTableScanDescData,
    _flags: u32,
) -> pg_sys::TableScanDesc

unsafe extern "C" fn scan_end(_desc: TableScanDesc)

unsafe extern "C" fn scan_getnextslot(
    _scan: TableScanDesc,
    _direction: ScanDirection::Type,
    _slot: *mut TupleTableSlot,
) -> bool

unsafe extern "C" fn tuple_satisfies_snapshot(
    _rel: Relation,
    _slot: *mut TupleTableSlot,
    _snapshot: Snapshot,
) -> bool


