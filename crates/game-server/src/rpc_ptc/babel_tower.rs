use super::*;

pub async fn on_rpc_get_babel_tower_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetBabelTowerDataArg>,
) -> Result<RpcGetBabelTowerDataRet, i32> {
    Ok(RpcGetBabelTowerDataRet::default())
}
