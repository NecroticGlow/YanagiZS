use super::*;

pub async fn on_rpc_get_arcade_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetArcadeDataArg>,
) -> Result<RpcGetArcadeDataRet, i32> {
    Ok(RpcGetArcadeDataRet::default())
}
