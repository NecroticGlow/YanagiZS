use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;

    pub async fn on_rpc_get_daily_challenge_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetDailyChallengeDataArg>,
    ) -> Result<RpcGetDailyChallengeDataRet, Retcode> {
        Ok(RpcGetDailyChallengeDataRet {
            retcode: Retcode::Succ,
            data: DailyChallengeData::default(),
        })
    }
}
