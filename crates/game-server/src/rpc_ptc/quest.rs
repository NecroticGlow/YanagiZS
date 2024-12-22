use super::*;

pub async fn on_rpc_get_quest_data_arg(
    ctx: &mut NetworkContext<'_, '_, RpcGetQuestDataArg>,
) -> Result<RpcGetQuestDataRet, i32> {
    Ok(RpcGetQuestDataRet {
        retcode: 0,
        quest_type: ctx.arg.quest_type,
        quest_data: QuestData::default(),
    })
}

pub async fn on_rpc_get_archive_data_arg(
    ctx: &mut NetworkContext<'_, '_, RpcGetArchiveDataArg>,
) -> Result<RpcGetArchiveDataRet, i32> {
    let archive_info = ctx.session.player_info.archive_info();

    Ok(RpcGetArchiveDataRet {
        retcode: 0,
        archive_data: ArchiveData {
            hollow_archive_id_list: archive_info
                .hollow_archive_id()
                .iter()
                .map(|id| *id as u32)
                .collect(),
            videotaps_info: archive_info
                .videotaps_info()
                .iter()
                .map(|(id, videotape)| VideotapeInfo {
                    archive_file_id: *id as u32,
                    finished: videotape.finished,
                })
                .collect(),
        },
    })
}

pub async fn on_rpc_get_hollow_data_arg(
    ctx: &mut NetworkContext<'_, '_, RpcGetHollowDataArg>,
) -> Result<RpcGetHollowDataRet, i32> {
    let yorozuya_info = ctx.session.player_info.yorozuya_info.as_ref().unwrap();

    Ok(RpcGetHollowDataRet {
        retcode: 0,
        hollow_data: HollowData {
            unlock_hollow_id_list: yorozuya_info
                .unlock_hollow_id()
                .iter()
                .map(|id| *id as u32)
                .collect(),
        },
    })
}

pub async fn on_rpc_get_fairy_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetFairyDataArg>,
) -> Result<RpcGetFairyDataRet, i32> {
    Ok(RpcGetFairyDataRet {
        retcode: 0,
        data: FairyData::default(),
    })
}

pub async fn on_rpc_check_yorozuya_info_refresh_arg(
    _: &mut NetworkContext<'_, '_, RpcCheckYorozuyaInfoRefreshArg>,
) -> Result<RpcCheckYorozuyaInfoRefreshRet, i32> {
    Ok(RpcCheckYorozuyaInfoRefreshRet::default())
}
