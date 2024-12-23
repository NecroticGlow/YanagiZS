use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;

    pub async fn on_rpc_get_battle_pass_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetBattlePassDataArg>,
    ) -> Result<RpcGetBattlePassDataRet, Retcode> {
        Ok(RpcGetBattlePassDataRet::default())
    }
}
