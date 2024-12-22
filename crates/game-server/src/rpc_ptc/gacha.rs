use super::*;

pub async fn on_rpc_get_gacha_data_arg(
    ctx: &mut NetworkContext<'_, '_, RpcGetGachaDataArg>,
) -> Result<RpcGetGachaDataRet, i32> {
    Ok(RpcGetGachaDataRet {
        gacha_type: ctx.arg.gacha_type,
        ..Default::default()
    })
}
