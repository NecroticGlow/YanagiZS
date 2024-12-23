use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use dungeon_info::BuddyUnitInfo;
    use item_info::ItemInfo;
    use protocol::util::build_client_dungeon_info;
    use tracing::debug;
    use util::build_client_scene_info;

    use crate::rpc_ptc::*;
    use std::collections::HashMap;

    pub async fn on_rpc_get_quest_data_arg(
        ctx: &mut NetworkContext<'_, RpcGetQuestDataArg>,
    ) -> Result<RpcGetQuestDataRet, Retcode> {
        let player_info = &ctx.session.player_info;

        let mut quest_collection_map = HashMap::new();
        player_info
            .quest_data()
            .quests()
            .iter()
            .for_each(|(collection_uid, id, quest_info)| {
                quest_collection_map
                    .entry(*collection_uid)
                    .or_insert_with(|| QuestCollection {
                        quest_type: quest_info.get_quest_type(),
                        ..Default::default()
                    })
                    .quest_id_list
                    .push(*id as u32);
            });

        Ok(RpcGetQuestDataRet {
            retcode: Retcode::Succ,
            quest_type: ctx.arg.quest_type,
            quest_data: QuestData {
                quest_collection_list: quest_collection_map.into_values().collect(),
            },
        })
    }

    pub async fn on_rpc_get_archive_data_arg(
        ctx: &mut NetworkContext<'_, RpcGetArchiveDataArg>,
    ) -> Result<RpcGetArchiveDataRet, Retcode> {
        let archive_info = ctx.session.player_info.archive_info();

        Ok(RpcGetArchiveDataRet {
            retcode: Retcode::Succ,
            archive_data: ArchiveData {
                hollow_archive_id_list: archive_info
                    .hollow_archive_id()
                    .iter()
                    .map(|id| *id as u32)
                    .collect(),
                videotaps_info: archive_info
                    .videotaps_info()
                    .iter()
                    .map(|(id, videotape)| VideotapeInfo {
                        archive_file_id: *id as u32,
                        finished: videotape.finished,
                    })
                    .collect(),
            },
        })
    }

    pub async fn on_rpc_get_hollow_data_arg(
        ctx: &mut NetworkContext<'_, RpcGetHollowDataArg>,
    ) -> Result<RpcGetHollowDataRet, Retcode> {
        let yorozuya_info = ctx.session.player_info.yorozuya_info.as_ref().unwrap();

        Ok(RpcGetHollowDataRet {
            retcode: Retcode::Succ,
            hollow_data: HollowData {
                unlock_hollow_id_list: yorozuya_info
                    .unlock_hollow_id()
                    .iter()
                    .map(|id| *id as u32)
                    .collect(),
            },
        })
    }

    pub async fn on_rpc_get_fairy_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetFairyDataArg>,
    ) -> Result<RpcGetFairyDataRet, Retcode> {
        Ok(RpcGetFairyDataRet {
            retcode: Retcode::Succ,
            data: FairyData::default(),
        })
    }

    pub async fn on_rpc_check_yorozuya_info_refresh_arg(
        _ctx: &mut NetworkContext<'_, RpcCheckYorozuyaInfoRefreshArg>,
    ) -> Result<RpcCheckYorozuyaInfoRefreshRet, Retcode> {
        Ok(RpcCheckYorozuyaInfoRefreshRet::default())
    }

    pub async fn on_rpc_begin_archive_battle_quest_arg(
        ctx: &mut NetworkContext<'_, RpcBeginArchiveBattleQuestArg>,
    ) -> Result<RpcBeginArchiveBattleQuestRet, Retcode> {
        let Some(archive_battle_template) = ctx
            .globals
            .filecfg
            .archive_battle_quest_template_tb
            .data()
            .unwrap_or_default()
            .iter()
            .find(|tmpl| tmpl.id() == ctx.arg.quest_id as i32)
        else {
            debug!(
                "archive battle quest with id {} is not defined",
                ctx.arg.quest_id
            );
            return Err(Retcode::Fail);
        };

        let player_info = &mut ctx.session.player_info;
        let mut avatar_uids = Vec::with_capacity(3);

        for robot_id in [
            archive_battle_template.slot_1_avatar(),
            archive_battle_template.slot_2_avatar(),
            archive_battle_template.slot_3_avatar(),
        ]
        .into_iter()
        .filter(|&id| id > 0)
        {
            let uid = ctx.session.uid_counter.next();
            player_info.items_mut().insert(
                uid,
                ItemInfo::AvatarInfo {
                    ngfmenjlddl: 6,
                    uid,
                    id: 0,
                    count: 1,
                    package: 0,
                    first_get_time: 0,
                    star: 0,
                    exp: 0,
                    level: 0,
                    rank: 0,
                    unlocked_talent_num: 0,
                    talent_switch: Vec::with_capacity(0),
                    skills: PropertyHashMap::Base(HashMap::with_capacity(0)),
                    is_custom_by_dungeon: true,
                    robot_id,
                },
            );

            avatar_uids.push(uid);
        }

        let dungeon_uid = ctx.session.uid_counter.next();
        let scene_uid = ctx.session.uid_counter.next();

        let cur_scene_uid = *player_info.scene_uid();
        let dungeon_info = protocol::dungeon_info::DungeonInfo {
            uid: dungeon_uid,
            id: ctx.arg.quest_id as i32,
            default_scene_uid: scene_uid,
            start_timestamp: time_util::unix_timestamp_ms(),
            to_be_destroyed: true,
            back_scene_uid: cur_scene_uid,
            quest_collection_uid: 0, // TODO: fetch the ArchiveFile/ArchiveBattle quest collection
            avatars: PropertyHashMap::Base(
                avatar_uids
                    .iter()
                    .map(|avatar_uid| {
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
            world_quest_id: ctx.arg.quest_id as i32,
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
            id: archive_battle_template.first_battle_event_id(),
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
            local_play_type: LocalPlayType::ArchiveBattle as u32,
            time: TimePeriodType::Morning,
            weather: WeatherType::SunShine,
        };

        let dungeon_collection = player_info.dungeon_collection_mut();
        dungeon_collection
            .dungeons_mut()
            .insert(dungeon_uid, dungeon_info);
        dungeon_collection
            .scenes_mut()
            .insert(scene_uid, scene_info);

        ctx.rpc_ptc
            .send_ptc(PtcEnterSceneArg {
                scene_info: build_client_scene_info(&ctx.session.player_info, scene_uid).unwrap(),
                dungeon_info: build_client_dungeon_info(&ctx.session.player_info, scene_uid),
            })
            .await;

        Ok(RpcBeginArchiveBattleQuestRet {
            retcode: Retcode::Succ,
            quest_id: ctx.arg.quest_id,
        })
    }

    pub async fn on_rpc_finish_archive_quest_arg(
        _ctx: &mut NetworkContext<'_, RpcFinishArchiveQuestArg>,
    ) -> Result<RpcFinishArchiveQuestRet, Retcode> {
        Ok(RpcFinishArchiveQuestRet::default())
    }
}
