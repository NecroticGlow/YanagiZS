use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;
    pub async fn on_rpc_get_vhs_store_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetVhsStoreDataArg>,
    ) -> Result<RpcGetVhsStoreDataRet, Retcode> {
        Ok(RpcGetVhsStoreDataRet {
            retcode: Retcode::Succ,
            data: VhsStoreData::default(),
        })
    }
}
