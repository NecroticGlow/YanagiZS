use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;

    use tracing::debug;

    use crate::level;

    pub async fn on_rpc_interact_with_client_entity_arg(
        ctx: &mut NetworkContext<'_, RpcInteractWithClientEntityArg>,
    ) -> Result<RpcInteractWithClientEntityRet, Retcode> {
        debug!("{:?}", &ctx.arg);
        Ok(RpcInteractWithClientEntityRet::default())
    }

    pub async fn on_rpc_interact_with_unit_arg(
        ctx: &mut NetworkContext<'_, RpcInteractWithUnitArg>,
    ) -> Result<RpcInteractWithUnitRet, Retcode> {
        debug!("{:?}", &ctx.arg);

        ctx.session
            .level_event_graph_mgr
            .begin_interact(ctx.arg.interaction, ctx.arg.npc_tag_id);

        level::fire_event(ctx.session, ctx.arg.interaction, "OnInteract");
        ctx.session
            .level_event_graph_mgr
            .sync_event_info(&ctx.rpc_ptc)
            .await;

        Ok(RpcInteractWithUnitRet::default())
    }

    pub async fn on_rpc_run_event_graph_arg(
        ctx: &mut NetworkContext<'_, RpcRunEventGraphArg>,
    ) -> Result<RpcRunEventGraphRet, Retcode> {
        ctx.rpc_ptc
            .send_ptc(PtcUpdateEventGraphArg {
                owner_type: ctx.arg.owner_type,
                tag: ctx.arg.tag,
                event_graph_uid: ctx.arg.event_graph_uid,
                npc_interaction: String::from("OnInteract"),
                is_event_success: true,
                event_graph_owner_uid: ctx.arg.owner_id,
            })
            .await;

        Ok(RpcRunEventGraphRet::default())
    }
}
