use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;
    use tracing::{debug, warn};

    use crate::scene_section_util;

    pub async fn on_rpc_get_ramen_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetRamenDataArg>,
    ) -> Result<RpcGetRamenDataRet, Retcode> {
        Ok(RpcGetRamenDataRet {
            retcode: Retcode::Succ,
            ramen_data: RamenData::default(),
        })
    }

    pub async fn on_rpc_get_cafe_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetCafeDataArg>,
    ) -> Result<RpcGetCafeDataRet, Retcode> {
        Ok(RpcGetCafeDataRet {
            retcode: Retcode::Succ,
            cafe_data: CafeData::default(),
        })
    }

    pub async fn on_rpc_get_reward_buff_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetRewardBuffDataArg>,
    ) -> Result<RpcGetRewardBuffDataRet, Retcode> {
        Ok(RpcGetRewardBuffDataRet {
            retcode: Retcode::Succ,
            data: RewardBuffData::default(),
        })
    }

    pub async fn on_rpc_get_red_dot_list_arg(
        _ctx: &mut NetworkContext<'_, RpcGetRedDotListArg>,
    ) -> Result<RpcGetRedDotListRet, Retcode> {
        Ok(RpcGetRedDotListRet::default())
    }

    pub async fn on_rpc_get_news_stand_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetNewsStandDataArg>,
    ) -> Result<RpcGetNewsStandDataRet, Retcode> {
        Ok(RpcGetNewsStandDataRet {
            retcode: Retcode::Succ,
            news_stand_data: NewsStandData::default(),
        })
    }

    pub async fn on_rpc_get_trashbin_hermit_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetTrashbinHermitDataArg>,
    ) -> Result<RpcGetTrashbinHermitDataRet, Retcode> {
        Ok(RpcGetTrashbinHermitDataRet {
            retcode: Retcode::Succ,
            trashbin_hermit_data: TrashbinHermitData::default(),
        })
    }

    pub async fn on_rpc_get_main_city_revival_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetMainCityRevivalDataArg>,
    ) -> Result<RpcGetMainCityRevivalDataRet, Retcode> {
        Ok(RpcGetMainCityRevivalDataRet {
            retcode: Retcode::Succ,
            main_city_revival_data: MainCityRevivalData::default(),
        })
    }

    pub async fn on_rpc_get_character_quest_list_arg(
        _ctx: &mut NetworkContext<'_, RpcGetCharacterQuestListArg>,
    ) -> Result<RpcGetCharacterQuestListRet, Retcode> {
        Ok(RpcGetCharacterQuestListRet::default())
    }

    pub async fn on_rpc_get_exploration_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetExplorationDataArg>,
    ) -> Result<RpcGetExplorationDataRet, Retcode> {
        Ok(RpcGetExplorationDataRet::default())
    }

    pub async fn on_rpc_get_miniscape_entrust_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetMiniscapeEntrustDataArg>,
    ) -> Result<RpcGetMiniscapeEntrustDataRet, Retcode> {
        Ok(RpcGetMiniscapeEntrustDataRet::default())
    }

    pub async fn on_rpc_get_journey_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetJourneyDataArg>,
    ) -> Result<RpcGetJourneyDataRet, Retcode> {
        Ok(RpcGetJourneyDataRet::default())
    }

    pub async fn on_rpc_get_photo_wall_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetPhotoWallDataArg>,
    ) -> Result<RpcGetPhotoWallDataRet, Retcode> {
        Ok(RpcGetPhotoWallDataRet::default())
    }

    pub async fn on_rpc_mod_time_arg(
        ctx: &mut NetworkContext<'_, RpcModTimeArg>,
    ) -> Result<RpcModTimeRet, Retcode> {
        debug!("{:?}", &ctx.arg);

        let player_info = &mut ctx.session.player_info;
        let scene_uid = *player_info.scene_uid();
        let dungeon_collection = player_info.dungeon_collection_mut();

        if let Some(protocol::scene_info::SceneInfo::Hall {
            main_city_time_info,
            ..
        }) = dungeon_collection.scenes_mut().get_mut(&scene_uid)
        {
            let prev_time = main_city_time_info.initial_time;
            main_city_time_info.initial_time = match ctx.arg.time_period {
                1 => 6 * 60,
                2 => 12 * 60,
                3 => 18 * 60,
                4 => 0 * 60,
                _ => 0,
            };

            if prev_time > main_city_time_info.initial_time {
                main_city_time_info.day_of_week = (main_city_time_info.day_of_week + 1) % 7;
            }

            let mut ptc =
                protocol::util::build_hall_refresh_arg(player_info, scene_uid, true).unwrap();
            scene_section_util::add_scene_units_to_hall_refresh_arg(
                ctx.session,
                scene_uid,
                &mut ptc,
            );
            ctx.rpc_ptc.send_ptc(ptc).await;
        } else {
            warn!("RpcModTime: currently not in Hall");
        }

        Ok(RpcModTimeRet {
            retcode: Retcode::Succ,
        })
    }

    pub async fn on_rpc_mod_main_city_avatar_arg(
        ctx: &mut NetworkContext<'_, RpcModMainCityAvatarArg>,
    ) -> Result<RpcModMainCityAvatarRet, Retcode> {
        debug!("{:?}", &ctx.arg);

        let player_info = &mut ctx.session.player_info;
        player_info.main_city_avatar_id = Some(ctx.arg.main_city_avatar_id);

        ctx.rpc_ptc
            .send_ptc(PtcPlayerSyncArg {
                basic_info: Some(protocol::util::build_player_basic_info(player_info)),
                ..Default::default()
            })
            .await;

        Ok(RpcModMainCityAvatarRet::default())
    }
}
