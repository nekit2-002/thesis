#import "../preamble.typ": file_listing

// = Реализация основных интерфейсов <fst_appendix>
// #file_listing("listings/tableam.rs", "rust", "Реализация TableAmRoutine на Rust") <lst:tableam>
// #file_listing("listings/trans.rs", "rust", "Связка типа TableAmArgs с типом internal") <lst:trans>
// #file_listing("listings/ops.rs", "rust", "Операции, описываемые структурой TupleTableSlotOps")<lst:ops>
// #file_listing("listings/insert.rs", "rust", "Метод вставки в таблицу")<lst:insert1>
// #file_listing("listings/scan-begin.rs", "rust", "Реализация scan_begin")<lst:scanbeg>
// #file_listing("listings/next-slot.rs", "rust", "Реализация scan_getnextslot")<lst:next-slot>
// #file_listing("listings/tuplesat.rs", "rust", "Реализаця tuple_satisfies_snapshot")<lst:tuplesat>
// #file_listing("listings/scan-end.rs", "rust", "реализация scan_end")<lst:scanend>

= Реализация функций взаимодействия с журналом предзаписи <fst_appendix>
#file_listing("listings/wal_stuff/rsam_redo_insert.rs", "rust","Реализация функции rsam_redo_insert")
#file_listing("listings/wal_stuff/rsam_redo_update.rs","rust", "Реализация функции rsam_redo_update")
#file_listing("listings/wal_stuff/rsam_redo_apply_tuple.rs","rust", "Реализация функции rsam_redo_apply_tuple")

// = Приложение 2