use common::time_util;
use dungeon_info::BuddyUnitInfo;
use item_info::ItemInfo;
use tracing::{debug, error};
use util::{build_client_dungeon_info, build_client_scene_info};

use crate::{level, scene_section_util};

use super::*;

pub async fn on_rpc_enter_world_arg(
    ctx: &mut NetworkContext<'_, '_, RpcEnterWorldArg>,
) -> Result<RpcEnterWorldRet, i32> {
    let player_info = &mut ctx.session.player_info;

    if *player_info.dungeon_collection().default_scene_uid() == 0 {
        let dungeon_uid = ctx.session.uid_counter.next();
        let scene_uid = ctx.session.uid_counter.next();

        let dungeon_info = protocol::dungeon_info::DungeonInfo {
            uid: dungeon_uid,
            id: 1,
            default_scene_uid: scene_uid,
            start_timestamp: time_util::unix_timestamp_ms(),
            to_be_destroyed: false,
            back_scene_uid: 0,
            quest_collection_uid: 0,
            avatars: phashmap![],
            buddy: BuddyUnitInfo {
                uid: 0,
                properties: 0,
            },
            world_quest_id: 0,
            scene_properties_uid: 0,
            drop_poll_chg_infos: phashmap![],
            is_in_dungeon: false,
            initiative_item: 0,
            initiative_item_used_times: 0,
            avatar_map: phashmap![],
            battle_report: Vec::new(),
            dungeon_group_uid: ctx.session.player_uid,
            entered_times: 0,
            is_preset_avatar: false,
            hollow_event_version: 0,
        };

        let scene_info = protocol::scene_info::SceneInfo::Hall {
            uid: scene_uid,
            id: 1,
            dungeon_uid,
            end_timestamp: 0,
            back_scene_uid: 0,
            entered_times: 1,
            section_id: 1,
            open_ui: UIType::Default,
            to_be_destroyed: false,
            camera_x: 0xFFFFFFFF,
            camera_y: 0xFFFFFFFF,
            main_city_time_info: scene_info::MainCityTimeInfo {
                initial_time: 60 * 21,
                day_of_week: 5,
                passed_milliseconds: 0,
                executing_event_groups: phashset![],
                unlocked_time_events: phashset![],
                time_event_groups_info: phashmap![],
                condition_progress_of_unlock: pdkhashmap![],
                condition_progress_of_end: pdkhashmap![],
                ended_time_events: phashset![],
                leave_time: 0,
            },
        };

        let dungeon_collection = player_info.dungeon_collection_mut();
        dungeon_collection
            .dungeons_mut()
            .insert(dungeon_uid, dungeon_info);
        dungeon_collection
            .scenes_mut()
            .insert(scene_uid, scene_info);

        *dungeon_collection.default_scene_uid_mut() = scene_uid;
    }

    let scene_uid = *ctx
        .session
        .player_info
        .dungeon_collection()
        .default_scene_uid();

    ctx.session.player_info.scene_uid = Some(scene_uid);

    if let Some(section_id) = ctx
        .session
        .player_info
        .dungeon_collection()
        .scenes()
        .get(&scene_uid)
        .map(|sc| *sc.get_section_id())
    {
        scene_section_util::init_hall_scene_section(ctx.session, scene_uid, section_id);
        level::on_section_enter(ctx.session, scene_uid, section_id);
    }

    let player_info = &mut ctx.session.player_info;
    player_info.second_last_enter_world_timestamp = player_info.last_enter_world_timestamp;
    player_info.last_enter_world_timestamp = Some(time_util::unix_timestamp_ms());

    let mut scene_info = build_client_scene_info(player_info, scene_uid).unwrap();
    scene_section_util::add_scene_units_to_scene_info(ctx.session, scene_uid, &mut scene_info);
    ctx.rpc_ptc
        .send_ptc(PtcEnterSceneArg {
            scene_info,
            dungeon_info: build_client_dungeon_info(&ctx.session.player_info, scene_uid),
        })
        .await;

    Ok(RpcEnterWorldRet::default())
}

pub async fn on_rpc_post_enter_world_arg(
    _: &mut NetworkContext<'_, '_, RpcPostEnterWorldArg>,
) -> Result<RpcPostEnterWorldRet, i32> {
    Ok(RpcPostEnterWorldRet::default())
}

pub async fn on_rpc_scene_transition_arg(
    _: &mut NetworkContext<'_, '_, RpcSceneTransitionArg>,
) -> Result<RpcSceneTransitionRet, i32> {
    Ok(RpcSceneTransitionRet::default())
}

pub async fn on_rpc_enter_section_complete_arg(
    _: &mut NetworkContext<'_, '_, RpcEnterSectionCompleteArg>,
) -> Result<RpcEnterSectionCompleteRet, i32> {
    Ok(RpcEnterSectionCompleteRet::default())
}

pub async fn on_rpc_save_pos_in_main_city_arg(
    ctx: &mut NetworkContext<'_, '_, RpcSavePosInMainCityArg>,
) -> Result<RpcSavePosInMainCityRet, i32> {
    let scene_uid = *ctx.session.player_info.scene_uid();
    let dungeon_collection = ctx.session.player_info.dungeon_collection();

    let Some(protocol::scene_info::SceneInfo::Hall { section_id, .. }) =
        dungeon_collection.scenes().get(&scene_uid)
    else {
        return Err(-1);
    };

    if *section_id == ctx.arg.section_id as i32 {
        if let (Ok(position), Ok(rotation)) = (
            ctx.arg.position.position.clone().try_into(),
            ctx.arg.position.rotation.clone().try_into(),
        ) {
            ctx.session.player_info.pos_in_main_city_mut().position = Some(position);
            ctx.session.player_info.pos_in_main_city_mut().rotation = Some(rotation);
            ctx.session
                .player_info
                .pos_in_main_city_mut()
                .initial_pos_id = Some(String::with_capacity(0));

            debug!(
                "player_uid: {}, pos in main city updated: {:?}",
                ctx.session.player_uid, ctx.arg,
            );
        } else {
            error!(
                "player_uid: {}, failed to save player pos: {:?}",
                ctx.session.player_uid, ctx.arg,
            );
        }
    }

    Ok(RpcSavePosInMainCityRet::default())
}

pub async fn on_rpc_enter_section_arg(
    ctx: &mut NetworkContext<'_, '_, RpcEnterSectionArg>,
) -> Result<RpcEnterSectionRet, i32> {
    let player_info = &mut ctx.session.player_info;
    let cur_scene_uid = *player_info.scene_uid();

    let dungeon_collection = player_info.dungeon_collection_mut();

    let Some(scene_info::SceneInfo::Hall { section_id, .. }) =
        dungeon_collection.scenes_mut().get_mut(&cur_scene_uid)
    else {
        error!("RpcEnterSection: current scene is not Hall!");
        return Err(-1);
    };

    *section_id = ctx.arg.section_id as i32;
    player_info.pos_in_main_city_mut().initial_pos_id = Some(ctx.arg.transform_id.clone());

    scene_section_util::init_hall_scene_section(
        ctx.session,
        cur_scene_uid,
        ctx.arg.section_id as i32,
    );
    level::on_section_enter(ctx.session, cur_scene_uid, ctx.arg.section_id as i32);

    let mut scene_info = build_client_scene_info(&ctx.session.player_info, cur_scene_uid).unwrap();
    scene_section_util::add_scene_units_to_scene_info(ctx.session, cur_scene_uid, &mut scene_info);
    ctx.rpc_ptc
        .send_ptc(PtcEnterSceneArg {
            scene_info,
            dungeon_info: build_client_dungeon_info(&ctx.session.player_info, cur_scene_uid),
        })
        .await;

    Ok(RpcEnterSectionRet::default())
}

pub async fn on_rpc_refresh_section_arg(
    _: &mut NetworkContext<'_, '_, RpcRefreshSectionArg>,
) -> Result<RpcRefreshSectionRet, i32> {
    Ok(RpcRefreshSectionRet {
        retcode: 0,
        refresh_status: HallRefreshStatus::Auto as u32,
    })
}

pub async fn on_rpc_begin_training_course_battle_arg(
    ctx: &mut NetworkContext<'_, '_, RpcBeginTrainingCourseBattleArg>,
) -> Result<RpcBeginTrainingCourseBattleRet, i32> {
    let player_info = &mut ctx.session.player_info;

    let dungeon_uid = ctx.session.uid_counter.next();
    let scene_uid = ctx.session.uid_counter.next();

    let cur_scene_uid = *player_info.scene_uid();
    let dungeon_info = protocol::dungeon_info::DungeonInfo {
        uid: dungeon_uid,
        id: 12254000,
        default_scene_uid: scene_uid,
        start_timestamp: time_util::unix_timestamp_ms(),
        to_be_destroyed: true,
        back_scene_uid: cur_scene_uid,
        quest_collection_uid: 0,
        avatars: PropertyHashMap::Base(
            ctx.arg
                .avatars
                .iter()
                .map(|avatar_id| {
                    let (avatar_uid, _) = player_info
                        .items
                        .as_ref()
                        .unwrap()
                        .iter()
                        .find(|(_, item)| {
                            if let ItemInfo::AvatarInfo { id, .. } = item {
                                (*id as u32) == *avatar_id
                            } else {
                                false
                            }
                        })
                        .unwrap();

                    (
                        *avatar_uid,
                        dungeon_info::AvatarUnitInfo {
                            uid: *avatar_uid,
                            properties_uid: 0,
                            hp_add_hollow: 0,
                            hp_lost_hollow: 0,
                            modified_property: pdkhashmap![],
                            layer_property_change: phashmap![],
                            is_banned: false,
                        },
                    )
                })
                .collect(),
        ),
        buddy: BuddyUnitInfo {
            uid: 0,
            properties: 0,
        },
        world_quest_id: 12254000,
        scene_properties_uid: 0,
        drop_poll_chg_infos: phashmap![],
        is_in_dungeon: false,
        initiative_item: 0,
        initiative_item_used_times: 0,
        avatar_map: phashmap![],
        battle_report: Vec::new(),
        dungeon_group_uid: ctx.session.player_uid,
        entered_times: 0,
        is_preset_avatar: false,
        hollow_event_version: 0,
    };

    let scene_info = protocol::scene_info::SceneInfo::Fight {
        uid: scene_uid,
        id: 19800014,
        dungeon_uid,
        end_timestamp: 0,
        back_scene_uid: cur_scene_uid,
        entered_times: 1,
        section_id: 0,
        open_ui: UIType::Default,
        to_be_destroyed: true,
        camera_x: 0xFFFFFFFF,
        camera_y: 0xFFFFFFFF,
        end_hollow: true,
        local_play_type: LocalPlayType::TrainingRoom as u32,
        time: TimePeriodType::Morning,
        weather: WeatherType::Rain,
    };

    let dungeon_collection = player_info.dungeon_collection_mut();
    dungeon_collection
        .dungeons_mut()
        .insert(dungeon_uid, dungeon_info);
    dungeon_collection
        .scenes_mut()
        .insert(scene_uid, scene_info);

    let mut scene_info = build_client_scene_info(&ctx.session.player_info, scene_uid).unwrap();
    scene_section_util::add_scene_units_to_scene_info(ctx.session, scene_uid, &mut scene_info);

    ctx.rpc_ptc
        .send_ptc(PtcEnterSceneArg {
            scene_info,
            dungeon_info: None,
        })
        .await;

    Ok(RpcBeginTrainingCourseBattleRet::default())
}

pub async fn on_rpc_battle_report_arg(
    _: &mut NetworkContext<'_, '_, RpcBattleReportArg>,
) -> Result<RpcBattleReportRet, i32> {
    Ok(RpcBattleReportRet::default())
}

pub async fn on_rpc_end_battle_arg(
    _: &mut NetworkContext<'_, '_, RpcEndBattleArg>,
) -> Result<RpcEndBattleRet, i32> {
    Ok(RpcEndBattleRet::default())
}

pub async fn on_rpc_leave_cur_scene_arg(
    ctx: &mut NetworkContext<'_, '_, RpcLeaveCurSceneArg>,
) -> Result<RpcLeaveCurSceneRet, i32> {
    let scene_uid = *ctx
        .session
        .player_info
        .dungeon_collection()
        .default_scene_uid();
    ctx.session.player_info.scene_uid = Some(scene_uid);

    if let Some(section_id) = ctx
        .session
        .player_info
        .dungeon_collection()
        .scenes()
        .get(&scene_uid)
        .map(|sc| *sc.get_section_id())
    {
        scene_section_util::init_hall_scene_section(ctx.session, scene_uid, section_id);
        level::on_section_enter(ctx.session, scene_uid, section_id);
    }

    let mut scene_info = build_client_scene_info(&ctx.session.player_info, scene_uid).unwrap();
    scene_section_util::add_scene_units_to_scene_info(ctx.session, scene_uid, &mut scene_info);
    ctx.rpc_ptc
        .send_ptc(PtcEnterSceneArg {
            scene_info,
            dungeon_info: build_client_dungeon_info(&ctx.session.player_info, scene_uid),
        })
        .await;

    Ok(RpcLeaveCurSceneRet::default())
}
