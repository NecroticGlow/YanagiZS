use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;

    pub async fn on_rpc_get_activity_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetActivityDataArg>,
    ) -> Result<RpcGetActivityDataRet, Retcode> {
        Ok(RpcGetActivityDataRet {
            retcode: Retcode::Succ,
            ..Default::default()
        })
    }

    pub async fn on_rpc_get_web_activity_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetWebActivityDataArg>,
    ) -> Result<RpcGetWebActivityDataRet, Retcode> {
        Ok(RpcGetWebActivityDataRet::default())
    }
}
