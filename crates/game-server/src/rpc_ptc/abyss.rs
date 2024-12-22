use super::*;

pub async fn on_rpc_abyss_get_data_arg(
    _: &mut NetworkContext<'_, '_, RpcAbyssGetDataArg>,
) -> Result<RpcAbyssGetDataRet, i32> {
    Ok(RpcAbyssGetDataRet {
        retcode: 0,
        abyss_info: AbyssInfo::default(),
    })
}

pub async fn on_rpc_abyss_arpeggio_get_data_arg(
    _: &mut NetworkContext<'_, '_, RpcAbyssArpeggioGetDataArg>,
) -> Result<RpcAbyssArpeggioGetDataRet, i32> {
    Ok(RpcAbyssArpeggioGetDataRet::default())
}

pub async fn on_rpc_get_abyss_reward_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetAbyssRewardDataArg>,
) -> Result<RpcGetAbyssRewardDataRet, i32> {
    Ok(RpcGetAbyssRewardDataRet {
        retcode: 0,
        abyss_reward_data: AbyssRewardData::default(),
    })
}
