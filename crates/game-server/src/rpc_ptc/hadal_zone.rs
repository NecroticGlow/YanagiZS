use super::*;

pub async fn on_rpc_get_hadal_zone_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetHadalZoneDataArg>,
) -> Result<RpcGetHadalZoneDataRet, i32> {
    Ok(RpcGetHadalZoneDataRet::default())
}
