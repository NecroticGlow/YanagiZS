use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;

    pub async fn on_rpc_abyss_get_data_arg(
        _ctx: &mut NetworkContext<'_, RpcAbyssGetDataArg>,
    ) -> Result<RpcAbyssGetDataRet, Retcode> {
        Ok(RpcAbyssGetDataRet {
            retcode: Retcode::Succ,
            abyss_info: AbyssInfo::default(),
        })
    }

    pub async fn on_rpc_abyss_arpeggio_get_data_arg(
        _ctx: &mut NetworkContext<'_, RpcAbyssArpeggioGetDataArg>,
    ) -> Result<RpcAbyssArpeggioGetDataRet, Retcode> {
        Ok(RpcAbyssArpeggioGetDataRet::default())
    }

    pub async fn on_rpc_get_abyss_reward_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetAbyssRewardDataArg>,
    ) -> Result<RpcGetAbyssRewardDataRet, Retcode> {
        Ok(RpcGetAbyssRewardDataRet {
            retcode: Retcode::Succ,
            abyss_reward_data: AbyssRewardData::default(),
        })
    }
}
