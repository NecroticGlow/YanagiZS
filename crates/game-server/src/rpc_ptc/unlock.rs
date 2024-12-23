use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;
    use item_info::ItemInfo;
    use tracing::debug;

    pub async fn on_rpc_get_tips_info_arg(
        _ctx: &mut NetworkContext<'_, RpcGetTipsInfoArg>,
    ) -> Result<RpcGetTipsInfoRet, Retcode> {
        Ok(RpcGetTipsInfoRet {
            retcode: Retcode::Succ,
            tips_info: TipsInfo::default(),
            ..Default::default()
        })
    }

    pub async fn on_rpc_get_client_systems_data_arg(
        ctx: &mut NetworkContext<'_, RpcGetClientSystemsDataArg>,
    ) -> Result<RpcGetClientSystemsDataRet, Retcode> {
        let player_info = &ctx.session.player_info;

        Ok(RpcGetClientSystemsDataRet {
            retcode: Retcode::Succ,
            data: ClientSystemsData {
                unlock_data: UnlockData {
                    unlocked_list: player_info
                        .unlock_info()
                        .unlocked_list()
                        .iter()
                        .copied()
                        .collect(),
                },
                post_girl_data: PostGirlData {
                    selected_post_girl_id_list: player_info
                        .selected_post_girl()
                        .iter()
                        .copied()
                        .collect(),
                    post_girl_item_list: player_info
                        .items()
                        .iter()
                        .map(|(_, item)| {
                            if let ItemInfo::PostGirlItem {
                                id, first_get_time, ..
                            } = item
                            {
                                Some(PostGirlItem {
                                    id: *id as u32,
                                    unlock_time: *first_get_time as i64,
                                })
                            } else {
                                None
                            }
                        })
                        .flatten()
                        .collect(),
                    show_random_selected: false,
                },
            },
        })
    }

    pub async fn on_rpc_get_private_message_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetPrivateMessageDataArg>,
    ) -> Result<RpcGetPrivateMessageDataRet, Retcode> {
        Ok(RpcGetPrivateMessageDataRet {
            retcode: Retcode::Succ,
            private_message_data: PrivateMessageData::default(),
        })
    }

    pub async fn on_rpc_get_collect_map_arg(
        _ctx: &mut NetworkContext<'_, RpcGetCollectMapArg>,
    ) -> Result<RpcGetCollectMapRet, Retcode> {
        Ok(RpcGetCollectMapRet {
            retcode: Retcode::Succ,
            collect_map: CollectMap::default(),
        })
    }

    pub async fn on_rpc_workbench_get_data_arg(
        _ctx: &mut NetworkContext<'_, RpcWorkbenchGetDataArg>,
    ) -> Result<RpcWorkbenchGetDataRet, Retcode> {
        Ok(RpcWorkbenchGetDataRet {
            retcode: Retcode::Succ,
            workbench_data: WorkbenchData::default(),
        })
    }

    pub async fn on_rpc_report_ui_layout_platform_arg(
        _ctx: &mut NetworkContext<'_, RpcReportUiLayoutPlatformArg>,
    ) -> Result<RpcReportUiLayoutPlatformRet, Retcode> {
        Ok(RpcReportUiLayoutPlatformRet::default())
    }

    pub async fn on_rpc_end_newbie_arg(
        _ctx: &mut NetworkContext<'_, RpcEndNewbieArg>,
    ) -> Result<RpcEndNewbieRet, Retcode> {
        Ok(RpcEndNewbieRet {
            retcode: Retcode::Succ,
        })
    }

    pub async fn on_rpc_select_post_girl_arg(
        ctx: &mut NetworkContext<'_, RpcSelectPostGirlArg>,
    ) -> Result<RpcSelectPostGirlRet, Retcode> {
        let player_info = &mut ctx.session.player_info;
        ctx.arg
            .new_selected_post_girl_id_list
            .iter()
            .try_for_each(|id| {
                player_info
                    .items()
                    .iter()
                    .any(|(_, i)| *i.get_id() == *id as i32)
                    .then_some(())
                    .ok_or_else(|| {
                        debug!("post_girl_id {id} is not unlocked");
                        Retcode::Fail
                    })
            })?;

        *player_info.selected_post_girl_mut() = PropertyHashSet::Base(
            ctx.arg
                .new_selected_post_girl_id_list
                .iter()
                .copied()
                .collect(),
        );

        ctx.rpc_ptc
            .send_ptc(PtcPlayerSyncArg {
                client_systems: Some(ClientSystemsSync {
                    post_girl_data: Some(protocol::util::build_post_girl_sync(player_info)),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .await;

        Ok(RpcSelectPostGirlRet {
            retcode: Retcode::Succ,
        })
    }
}
