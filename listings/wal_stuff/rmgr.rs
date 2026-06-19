const RsAmRmgr: RmgrData = RmgrData {
    rm_name: "rsam resource manager".as_ptr().cast(),
    rm_redo: Some(rsam_redo),
    rm_desc: Some(rsam_desc),
    rm_identify: Some(rsam_identify),
    rm_startup: None,
    rm_cleanup: None,
    rm_mask: None,
    rm_decode: None,
};
