use super::*;

pub async fn on_rpc_get_activity_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetActivityDataArg>,
) -> Result<RpcGetActivityDataRet, i32> {
    Ok(RpcGetActivityDataRet {
        retcode: 0,
        ..Default::default()
    })
}

pub async fn on_rpc_get_web_activity_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetWebActivityDataArg>,
) -> Result<RpcGetWebActivityDataRet, i32> {
    Ok(RpcGetWebActivityDataRet::default())
}
