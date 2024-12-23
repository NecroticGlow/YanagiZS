use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;

    pub async fn on_rpc_get_camp_idle_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetCampIdleDataArg>,
    ) -> Result<RpcGetCampIdleDataRet, Retcode> {
        Ok(RpcGetCampIdleDataRet {
            retcode: Retcode::Succ,
            camp_idle_data: CampIdleData::default(),
        })
    }
}
