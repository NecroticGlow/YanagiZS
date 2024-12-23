use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;

    pub async fn on_rpc_perform_trigger_arg(
        ctx: &mut NetworkContext<'_, RpcPerformTriggerArg>,
    ) -> Result<RpcPerformTriggerRet, Retcode> {
        Ok(RpcPerformTriggerRet {
            retcode: Retcode::Succ,
            perform_uid: ctx.session.uid_counter.next() as i64,
        })
    }

    pub async fn on_rpc_perform_jump_arg(
        _ctx: &mut NetworkContext<'_, RpcPerformJumpArg>,
    ) -> Result<RpcPerformJumpRet, Retcode> {
        Ok(RpcPerformJumpRet {
            retcode: Retcode::Succ,
        })
    }

    pub async fn on_rpc_perform_end_arg(
        _ctx: &mut NetworkContext<'_, RpcPerformEndArg>,
    ) -> Result<RpcPerformEndRet, Retcode> {
        Ok(RpcPerformEndRet {
            retcode: Retcode::Succ,
        })
    }
}
