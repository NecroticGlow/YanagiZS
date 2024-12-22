use super::*;

pub async fn on_rpc_get_daily_challenge_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetDailyChallengeDataArg>,
) -> Result<RpcGetDailyChallengeDataRet, i32> {
    Ok(RpcGetDailyChallengeDataRet {
        retcode: 0,
        data: DailyChallengeData::default(),
    })
}
