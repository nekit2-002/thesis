#import "../preamble.typ": file_listing

= Реализация основных интерфейсов <fst_appendix>
#file_listing("listings/handler_stuff/tableam.rs", "rust", "Реализация TableAmRoutine на Rust") <lst:tableam>
#file_listing("listings/handler_stuff/trans.rs", "rust", "Связка типа TableAmArgs с типом internal") <lst:trans>
#file_listing("listings/slot_stuff/ops.rs", "rust", "Операции, описываемые структурой TupleTableSlotOps")<lst:ops>
#file_listing("listings/slot_stuff/rsamops.rs", "rust", "Static переменная, описывающая коллбеки слота")<lst:rsamops>
#file_listing("listings/slot_stuff/rsam_deform_tuple.rs", "rust", "Функция для трансляции атрибутов в массив Datum объектов")<lst:rsam_deform_tuple>
// #file_listing("listings/slot_stuff/rsamtuple.rs", "rust", "Основной тип, описывающий структуру кортежа
// метода доступа rsam")<lst:rsam_tuple>
#file_listing("listings/handler_stuff/insert.rs", "rust", "Метод вставки в таблицу")<lst:insert1>
#file_listing("listings/handler_stuff/scan-begin.rs", "rust", "Реализация scan_begin")<lst:scanbeg>
#file_listing("listings/handler_stuff/next-slot.rs", "rust", "Реализация scan_getnextslot")<lst:next-slot>
#file_listing("listings/handler_stuff/tuplesat.rs", "rust", "Реализаця tuple_satisfies_snapshot")<lst:tuplesat>
#file_listing("listings/handler_stuff/scan-end.rs", "rust", "реализация scan_end")<lst:scanend>

= Реализация функций взаимодействия с журналом предзаписи <snd_appendix>
#file_listing("listings/wal_stuff/rsam_redo_insert.rs", "rust","Реализация функции rsam_redo_insert")
#file_listing("listings/wal_stuff/rsam_redo_update.rs","rust", "Реализация функции rsam_redo_update")
#file_listing("listings/wal_stuff/rsam_redo_apply_tuple.rs","rust", "Реализация функции rsam_redo_apply_tuple")

// = Приложение 2