use super::*;

pub async fn on_rpc_get_fishing_contest_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetFishingContestDataArg>,
) -> Result<RpcGetFishingContestDataRet, i32> {
    Ok(RpcGetFishingContestDataRet::default())
}
