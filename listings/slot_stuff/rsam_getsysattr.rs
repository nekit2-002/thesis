#[pg_guard]
unsafe extern "C-unwind" fn tts_rsam_getsysattr(
    slot: *mut TupleTableSlot,
    attnum: i32,
    tupleDesc: TupleDesc,
    isnull: *mut bool,
) -> Datum {
    let rsamslot = slot as *mut RsAmTupleTableSlot;
    rsam_getsysattr((*rsamslot).tuple, attnum, (*slot).tts_TupleDescriptor, isnull)
}

#[pg_guard]
unsafe extern "C-unwind" fn rsam_getsysattr(
    tup: RsAmTuple,
    attnum: i32,
    tupleDesc: TupleDesc,
    isnull: *mut bool
) -> Datum {
    *isnull = false;
    let result = match attnum {
        SelfItemPointerAttributeNumber => {
            PointerGetDatum(&raw mut (*tup).t_self)
        },
        MinTransactionIdAttributeNumber => {
            TransactionIdGetDatum(RsAmTupleGetXMinFromUndo(tup))
        },
        MaxTransactionIdAttributeNumber => {
            TransactionIdGetDatum(RsAmTupleHeaderGetRawXmax((*tup).t_data))
        },
        TableOidAttributeNumber => {
            ObjectIdGetDatum((*tup).t_data)
        }

        _ => 0
    }

    result
}