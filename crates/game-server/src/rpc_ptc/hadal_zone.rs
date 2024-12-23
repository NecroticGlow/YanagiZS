use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;

    pub async fn on_rpc_get_hadal_zone_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetHadalZoneDataArg>,
    ) -> Result<RpcGetHadalZoneDataRet, Retcode> {
        Ok(RpcGetHadalZoneDataRet::default())
    }
}
