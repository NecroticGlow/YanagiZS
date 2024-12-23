use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;

    pub async fn on_rpc_get_embattles_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetEmbattlesDataArg>,
    ) -> Result<RpcGetEmbattlesDataRet, Retcode> {
        Ok(RpcGetEmbattlesDataRet {
            retcode: Retcode::Succ,
            embattles_data: EmbattlesData::default(),
        })
    }

    pub async fn on_rpc_report_embattle_info_arg(
        _ctx: &mut NetworkContext<'_, RpcReportEmbattleInfoArg>,
    ) -> Result<RpcReportEmbattleInfoRet, Retcode> {
        Ok(RpcReportEmbattleInfoRet::default())
    }
}
