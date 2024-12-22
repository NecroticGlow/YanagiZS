use tracing::{debug, instrument};

use crate::level;

use super::*;

#[instrument(skip_all)]
pub async fn on_rpc_interact_with_client_entity_arg(
    ctx: &mut NetworkContext<'_, '_, RpcInteractWithClientEntityArg>,
) -> Result<RpcInteractWithClientEntityRet, i32> {
    debug!("{:?}", &ctx.arg);
    Ok(RpcInteractWithClientEntityRet::default())
}

#[instrument(skip_all)]
pub async fn on_rpc_interact_with_unit_arg(
    ctx: &mut NetworkContext<'_, '_, RpcInteractWithUnitArg>,
) -> Result<RpcInteractWithUnitRet, i32> {
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
    ctx: &mut NetworkContext<'_, '_, RpcRunEventGraphArg>,
) -> Result<RpcRunEventGraphRet, i32> {
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
