use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;

    pub async fn on_rpc_get_gacha_data_arg(
        ctx: &mut NetworkContext<'_, RpcGetGachaDataArg>,
    ) -> Result<RpcGetGachaDataRet, Retcode> {
        Ok(RpcGetGachaDataRet {
            gacha_type: ctx.arg.gacha_type,
            ..Default::default()
        })
    }
}
