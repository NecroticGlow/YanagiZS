use super::*;

pub async fn on_rpc_get_ridus_got_boo_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetRidusGotBooDataArg>,
) -> Result<RpcGetRidusGotBooDataRet, i32> {
    Ok(RpcGetRidusGotBooDataRet::default())
}
