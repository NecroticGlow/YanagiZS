use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;

    pub async fn on_rpc_get_arcade_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetArcadeDataArg>,
    ) -> Result<RpcGetArcadeDataRet, Retcode> {
        Ok(RpcGetArcadeDataRet::default())
    }
}
