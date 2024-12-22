use common::time_util;

use super::*;

pub async fn on_rpc_get_player_basic_info_arg(
    ctx: &mut NetworkContext<'_, '_, RpcGetPlayerBasicInfoArg>,
) -> Result<RpcGetPlayerBasicInfoRet, i32> {
    Ok(RpcGetPlayerBasicInfoRet {
        retcode: 0,
        basic_info: protocol::util::build_player_basic_info(&ctx.session.player_info),
    })
}

pub async fn on_ptc_keep_alive_arg(_: &mut NetworkContext<'_, '_, PtcKeepAliveArg>) {
    ()
}

pub async fn on_rpc_get_server_timestamp_arg(
    _: &mut NetworkContext<'_, '_, RpcGetServerTimestampArg>,
) -> Result<RpcGetServerTimestampRet, i32> {
    Ok(RpcGetServerTimestampRet {
        retcode: 0,
        utc_offset: 3,
        timestamp: time_util::unix_timestamp(),
    })
}

pub async fn on_rpc_video_get_info_arg(
    _: &mut NetworkContext<'_, '_, RpcVideoGetInfoArg>,
) -> Result<RpcVideoGetInfoRet, i32> {
    Ok(RpcVideoGetInfoRet { retcode: 0 })
}

pub async fn on_rpc_get_authkey_arg(
    _: &mut NetworkContext<'_, '_, RpcGetAuthkeyArg>,
) -> Result<RpcGetAuthkeyRet, i32> {
    Ok(RpcGetAuthkeyRet::default())
}

pub async fn on_rpc_save_player_system_setting_arg(
    ctx: &mut NetworkContext<'_, '_, RpcSavePlayerSystemSettingArg>,
) -> Result<RpcSavePlayerSystemSettingRet, i32> {
    tracing::info!("save_player_system_setting: {:?}", &ctx.arg);

    Ok(RpcSavePlayerSystemSettingRet { retcode: 0 })
}

pub async fn on_rpc_get_player_mails_arg(
    _: &mut NetworkContext<'_, '_, RpcGetPlayerMailsArg>,
) -> Result<RpcGetPlayerMailsRet, i32> {
    Ok(RpcGetPlayerMailsRet::default())
}

pub async fn on_rpc_get_role_card_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetRoleCardDataArg>,
) -> Result<RpcGetRoleCardDataRet, i32> {
    Ok(RpcGetRoleCardDataRet {
        retcode: 0,
        role_card_data: RoleCardData::default(),
    })
}

pub async fn on_rpc_get_month_card_reward_list_arg(
    _: &mut NetworkContext<'_, '_, RpcGetMonthCardRewardListArg>,
) -> Result<RpcGetMonthCardRewardListRet, i32> {
    Ok(RpcGetMonthCardRewardListRet::default())
}

pub async fn on_rpc_get_display_case_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetDisplayCaseDataArg>,
) -> Result<RpcGetDisplayCaseDataRet, i32> {
    Ok(RpcGetDisplayCaseDataRet::default())
}

pub async fn on_rpc_player_operation_arg(
    _: &mut NetworkContext<'_, '_, RpcPlayerOperationArg>,
) -> Result<RpcPlayerOperationRet, i32> {
    Ok(RpcPlayerOperationRet::default())
}

pub async fn on_rpc_player_transaction_arg(
    ctx: &mut NetworkContext<'_, '_, RpcPlayerTransactionArg>,
) -> Result<RpcPlayerTransactionRet, i32> {
    let player_uid = ctx.session.player_info.uid();
    let scene_uid = ctx.session.player_info.scene_uid();

    Ok(RpcPlayerTransactionRet {
        retcode: 0,
        transaction: format!("{player_uid}-{scene_uid}"),
    })
}

pub async fn on_rpc_get_player_network_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetPlayerNetworkDataArg>,
) -> Result<RpcGetPlayerNetworkDataRet, i32> {
    Ok(RpcGetPlayerNetworkDataRet {
        retcode: 0,
        player_network_data: Some(PlayerNetworkData::default()),
    })
}

pub async fn on_rpc_set_language_arg(
    _: &mut NetworkContext<'_, '_, RpcSetLanguageArg>,
) -> Result<RpcSetLanguageRet, i32> {
    Ok(RpcSetLanguageRet::default())
}
