use tracing::{debug, warn};

use crate::scene_section_util;

use super::*;

pub async fn on_rpc_get_ramen_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetRamenDataArg>,
) -> Result<RpcGetRamenDataRet, i32> {
    Ok(RpcGetRamenDataRet {
        retcode: 0,
        ramen_data: RamenData::default(),
    })
}

pub async fn on_rpc_get_cafe_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetCafeDataArg>,
) -> Result<RpcGetCafeDataRet, i32> {
    Ok(RpcGetCafeDataRet {
        retcode: 0,
        cafe_data: CafeData::default(),
    })
}

pub async fn on_rpc_get_reward_buff_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetRewardBuffDataArg>,
) -> Result<RpcGetRewardBuffDataRet, i32> {
    Ok(RpcGetRewardBuffDataRet {
        retcode: 0,
        data: RewardBuffData::default(),
    })
}

pub async fn on_rpc_get_red_dot_list_arg(
    _: &mut NetworkContext<'_, '_, RpcGetRedDotListArg>,
) -> Result<RpcGetRedDotListRet, i32> {
    Ok(RpcGetRedDotListRet::default())
}

pub async fn on_rpc_get_news_stand_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetNewsStandDataArg>,
) -> Result<RpcGetNewsStandDataRet, i32> {
    Ok(RpcGetNewsStandDataRet {
        retcode: 0,
        news_stand_data: NewsStandData::default(),
    })
}

pub async fn on_rpc_get_trashbin_hermit_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetTrashbinHermitDataArg>,
) -> Result<RpcGetTrashbinHermitDataRet, i32> {
    Ok(RpcGetTrashbinHermitDataRet {
        retcode: 0,
        trashbin_hermit_data: TrashbinHermitData::default(),
    })
}

pub async fn on_rpc_get_main_city_revival_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetMainCityRevivalDataArg>,
) -> Result<RpcGetMainCityRevivalDataRet, i32> {
    Ok(RpcGetMainCityRevivalDataRet {
        retcode: 0,
        main_city_revival_data: MainCityRevivalData::default(),
    })
}

pub async fn on_rpc_get_character_quest_list_arg(
    _: &mut NetworkContext<'_, '_, RpcGetCharacterQuestListArg>,
) -> Result<RpcGetCharacterQuestListRet, i32> {
    Ok(RpcGetCharacterQuestListRet::default())
}

pub async fn on_rpc_get_exploration_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetExplorationDataArg>,
) -> Result<RpcGetExplorationDataRet, i32> {
    Ok(RpcGetExplorationDataRet::default())
}

pub async fn on_rpc_get_miniscape_entrust_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetMiniscapeEntrustDataArg>,
) -> Result<RpcGetMiniscapeEntrustDataRet, i32> {
    Ok(RpcGetMiniscapeEntrustDataRet::default())
}

pub async fn on_rpc_get_journey_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetJourneyDataArg>,
) -> Result<RpcGetJourneyDataRet, i32> {
    Ok(RpcGetJourneyDataRet::default())
}

pub async fn on_rpc_get_photo_wall_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetPhotoWallDataArg>,
) -> Result<RpcGetPhotoWallDataRet, i32> {
    Ok(RpcGetPhotoWallDataRet::default())
}

pub async fn on_rpc_mod_time_arg(
    ctx: &mut NetworkContext<'_, '_, RpcModTimeArg>,
) -> Result<RpcModTimeRet, i32> {
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

        let mut ptc = protocol::util::build_hall_refresh_arg(player_info, scene_uid, true).unwrap();
        scene_section_util::add_scene_units_to_hall_refresh_arg(ctx.session, scene_uid, &mut ptc);
        ctx.rpc_ptc.send_ptc(ptc).await;
    } else {
        warn!("RpcModTime: currently not in Hall");
    }

    Ok(RpcModTimeRet { retcode: 0 })
}

pub async fn on_rpc_mod_main_city_avatar_arg(
    ctx: &mut NetworkContext<'_, '_, RpcModMainCityAvatarArg>,
) -> Result<RpcModMainCityAvatarRet, i32> {
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
