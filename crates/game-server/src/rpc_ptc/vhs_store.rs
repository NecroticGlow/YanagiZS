use super::*;

pub async fn on_rpc_get_vhs_store_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetVhsStoreDataArg>,
) -> Result<RpcGetVhsStoreDataRet, i32> {
    Ok(RpcGetVhsStoreDataRet {
        retcode: 0,
        data: VhsStoreData::default(),
    })
}
