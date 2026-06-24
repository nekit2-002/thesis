#import "../preamble.typ": file_listing

= Приложение 1 <fst_appendix>
#file_listing("listings/handler_stuff/tableam.rs", "rust", "Реализация TableAmRoutine на Rust") <lst:tableam>
#file_listing("listings/handler_stuff/trans.rs", "rust", "Связка типа TableAmArgs с типом internal") <lst:trans>
#file_listing("listings/slot_stuff/ops.rs", "rust", "Операции, описываемые структурой TupleTableSlotOps")<lst:ops>
#file_listing("listings/slot_stuff/rsamops.rs", "rust", "Static переменная, описывающая коллбеки слота")<lst:rsamops>
#file_listing("listings/slot_stuff/rsam_deform_tuple.rs", "rust", "Функция для трансляции атрибутов в массив Datum объектов")<lst:rsam_deform_tuple>
#file_listing("listings/slot_stuff/rsam_getsysattr.rs", "rust", "Функция для получения системного атрибута")<lst:rsam_getsysattr>
#file_listing("listings/handler_stuff/insert.rs", "rust", "Метод вставки в таблицу")<lst:insert1>
#file_listing("listings/insert/heap-insert.rs", "rust", "Функция-помощник вставки rsam_insert")<lst:insert2>
#file_listing("listings/handler_stuff/scan-begin.rs", "rust", "Реализация scan_begin")<lst:scanbeg>
#file_listing("listings/handler_stuff/next-slot.rs", "rust", "Реализация scan_getnextslot")<lst:next-slot>
#file_listing("listings/handler_stuff/tuplesat.rs", "rust", "Реализаця tuple_satisfies_snapshot")<lst:tuplesat>
#file_listing("listings/scan/tuple_fetch_row_version.rs", "rust", "Реализаця tuple_fetch_row_version")<lst:tuplefetchrow>
#file_listing("listings/scan/tuple_fetch.rs", "rust", "Реализаця вспомогательной функции tuple_fetch")<lst:tuplefetch>
#file_listing("listings/handler_stuff/scan-end.rs", "rust", "Реализация scan_end")<lst:scanend>
#file_listing("listings/delete/rsam_delete.rs", "rust", "Реализация tuple_delete")<lst:delete>

= Приложение 2 <snd_appendix>
#file_listing("listings/wal_stuff/rsam_redo.rs", "rust","Реализация коллбека rsam_redo")<lst:rsam_redo>
#file_listing("listings/wal_stuff/rsam_redo_insert.rs", "rust","Реализация функции rsam_redo_insert")<lst:rsam_redo_insert>
#file_listing("listings/wal_stuff/rsam_identify.rs","rust", "Реализация коллбека rsam_identify")<lst:rsam_identify>
#file_listing("listings/wal_stuff/rsam_desc.rs", "rust", "Реализация коллбека rsam_desc")<lst:rsam_desc>
// #file_listing("listings/wal_stuff/rsam_redo_apply_tuple.rs","rust", "Реализация функции rsam_redo_apply_tuple")

// = Приложение 2