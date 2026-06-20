#[pg_guard]
unsafe extern "C-unwind" fn slot_deform_rsam_tuple(
    slot: *mut TupleTableSlot,
    tuple: RsAmTuple,
    offp: *mut u32,
    natts: i32,
) {
    let tupleDesc = (*slot).tts_tupleDescriptor;
    let values = (*slot).tts_values;
    let isnull = (*slot).tts_isnull;
    let tup = (*tuple).t_data;

    let hasnulls = RsAmTupleHasNulls(tuple);
    let mut attnum = (*slot).tts_nvalid;
    let bp = (*tup).t_bits;

    natts = Min(RsAmTupleHeaderGetNatts(tup), natts);

    let mut off = if attnum == 0 { 0 } else { *offp as usize };

    let mut tp = tup as *mut i8 + (*tup).h_off + off;

    while attnum < natts {
        let thisatt = TupleDescAttr(tupleDesc, attnum);

        if hasnulls && att_isnull(attnum, bp) {
            values[attnum] = 0 as Datum;
            isnull[attnum] = true;
            attnum += 1;
            continue;
        }

        isnull[attnum] = false;

        if (*thisatt).attlen == -1 {
            tp = att_align_pointer(tp, (*thisatt).attalign, -1, tp);
        } else if !(*thisatt).attbyval {
            tp = att_align_nominal(tp, (*thisatt).attalign);
        }

        if (*thisatt).attbyval {
            let mut datum = Datum { std::ptr::null_mut() };
            datum.copy_from_slice(tp, (*thisatt).attlen);

            let datum = fetch_att(datum, true, (*thisatt).attlen);
            values[attnum] = datum;
        } else {
            values[attnum] = PointerGetDatum(tp);
        }

        tp = att_addlength_pointer(tp, thisatt.attlen, tp);
        attnum += 1;
    }

    // Save state for next execution
    slot.tts_nvalid = attnum;
    *offp = new_off;
}
