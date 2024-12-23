use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;
    pub async fn on_rpc_get_ridus_got_boo_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetRidusGotBooDataArg>,
    ) -> Result<RpcGetRidusGotBooDataRet, Retcode> {
        Ok(RpcGetRidusGotBooDataRet::default())
    }
}
