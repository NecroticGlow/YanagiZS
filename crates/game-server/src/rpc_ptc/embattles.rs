use super::*;

pub async fn on_rpc_get_embattles_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetEmbattlesDataArg>,
) -> Result<RpcGetEmbattlesDataRet, i32> {
    Ok(RpcGetEmbattlesDataRet {
        retcode: 0,
        embattles_data: EmbattlesData::default(),
    })
}

pub async fn on_rpc_report_embattle_info_arg(
    _: &mut NetworkContext<'_, '_, RpcReportEmbattleInfoArg>,
) -> Result<RpcReportEmbattleInfoRet, i32> {
    Ok(RpcReportEmbattleInfoRet::default())
}
