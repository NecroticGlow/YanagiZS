use super::*;

pub async fn on_rpc_get_battle_pass_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetBattlePassDataArg>,
) -> Result<RpcGetBattlePassDataRet, i32> {
    Ok(RpcGetBattlePassDataRet::default())
}
