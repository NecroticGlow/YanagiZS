use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;

    use common::time_util;
    pub async fn on_rpc_get_player_basic_info_arg(
        ctx: &mut NetworkContext<'_, RpcGetPlayerBasicInfoArg>,
    ) -> Result<RpcGetPlayerBasicInfoRet, Retcode> {
        Ok(RpcGetPlayerBasicInfoRet {
            retcode: Retcode::Succ,
            basic_info: protocol::util::build_player_basic_info(&ctx.session.player_info),
        })
    }

    pub async fn on_ptc_keep_alive_arg(_ctx: &mut NetworkContext<'_, protocol::PtcKeepAliveArg>) {
        ()
    }

    pub async fn on_rpc_get_server_timestamp_arg(
        _ctx: &mut NetworkContext<'_, RpcGetServerTimestampArg>,
    ) -> Result<RpcGetServerTimestampRet, Retcode> {
        Ok(RpcGetServerTimestampRet {
            retcode: Retcode::Succ,
            utc_offset: 3,
            timestamp: time_util::unix_timestamp(),
        })
    }

    pub async fn on_rpc_video_get_info_arg(
        _ctx: &mut NetworkContext<'_, RpcVideoGetInfoArg>,
    ) -> Result<RpcVideoGetInfoRet, Retcode> {
        Ok(RpcVideoGetInfoRet {
            retcode: Retcode::Succ,
        })
    }

    pub async fn on_rpc_get_authkey_arg(
        _ctx: &mut NetworkContext<'_, RpcGetAuthkeyArg>,
    ) -> Result<RpcGetAuthkeyRet, Retcode> {
        Ok(RpcGetAuthkeyRet::default())
    }

    pub async fn on_rpc_save_player_system_setting_arg(
        ctx: &mut NetworkContext<'_, RpcSavePlayerSystemSettingArg>,
    ) -> Result<RpcSavePlayerSystemSettingRet, Retcode> {
        tracing::info!("save_player_system_setting: {:?}", &ctx.arg);

        Ok(RpcSavePlayerSystemSettingRet {
            retcode: Retcode::Succ,
        })
    }

    pub async fn on_rpc_get_player_mails_arg(
        _ctx: &mut NetworkContext<'_, RpcGetPlayerMailsArg>,
    ) -> Result<RpcGetPlayerMailsRet, Retcode> {
        Ok(RpcGetPlayerMailsRet::default())
    }

    pub async fn on_rpc_get_role_card_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetRoleCardDataArg>,
    ) -> Result<RpcGetRoleCardDataRet, Retcode> {
        Ok(RpcGetRoleCardDataRet {
            retcode: Retcode::Succ,
            role_card_data: RoleCardData::default(),
        })
    }

    pub async fn on_rpc_get_month_card_reward_list_arg(
        _ctx: &mut NetworkContext<'_, RpcGetMonthCardRewardListArg>,
    ) -> Result<RpcGetMonthCardRewardListRet, Retcode> {
        Ok(RpcGetMonthCardRewardListRet::default())
    }

    pub async fn on_rpc_get_display_case_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetDisplayCaseDataArg>,
    ) -> Result<RpcGetDisplayCaseDataRet, Retcode> {
        Ok(RpcGetDisplayCaseDataRet::default())
    }

    pub async fn on_rpc_player_operation_arg(
        _ctx: &mut NetworkContext<'_, RpcPlayerOperationArg>,
    ) -> Result<RpcPlayerOperationRet, Retcode> {
        Ok(RpcPlayerOperationRet::default())
    }

    pub async fn on_rpc_player_transaction_arg(
        ctx: &mut NetworkContext<'_, RpcPlayerTransactionArg>,
    ) -> Result<RpcPlayerTransactionRet, Retcode> {
        let player_uid = ctx.session.player_info.uid();
        let scene_uid = ctx.session.player_info.scene_uid();

        Ok(RpcPlayerTransactionRet {
            retcode: Retcode::Succ,
            transaction: format!("{player_uid}-{scene_uid}"),
        })
    }

    pub async fn on_rpc_get_player_network_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetPlayerNetworkDataArg>,
    ) -> Result<RpcGetPlayerNetworkDataRet, Retcode> {
        Ok(RpcGetPlayerNetworkDataRet {
            retcode: Retcode::Succ,
            player_network_data: Some(PlayerNetworkData::default()),
        })
    }

    pub async fn on_rpc_set_language_arg(
        _ctx: &mut NetworkContext<'_, RpcSetLanguageArg>,
    ) -> Result<RpcSetLanguageRet, Retcode> {
        Ok(RpcSetLanguageRet::default())
    }
}
