use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;

    pub async fn on_rpc_get_fishing_contest_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetFishingContestDataArg>,
    ) -> Result<RpcGetFishingContestDataRet, Retcode> {
        Ok(RpcGetFishingContestDataRet::default())
    }
}
