use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;

    pub async fn on_rpc_get_babel_tower_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetBabelTowerDataArg>,
    ) -> Result<RpcGetBabelTowerDataRet, Retcode> {
        Ok(RpcGetBabelTowerDataRet::default())
    }
}
