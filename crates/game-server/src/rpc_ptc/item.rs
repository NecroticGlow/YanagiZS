use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;
    use item_info::ItemInfo;
    use tracing::debug;

    pub async fn on_rpc_get_weapon_data_arg(
        ctx: &mut NetworkContext<'_, RpcGetWeaponDataArg>,
    ) -> Result<RpcGetWeaponDataRet, Retcode> {
        Ok(RpcGetWeaponDataRet {
            retcode: Retcode::Succ,
            weapon_list: protocol::util::build_sync_weapon_info_list(&ctx.session.player_info),
        })
    }

    pub async fn on_rpc_get_equip_data_arg(
        ctx: &mut NetworkContext<'_, RpcGetEquipDataArg>,
    ) -> Result<RpcGetEquipDataRet, Retcode> {
        Ok(RpcGetEquipDataRet {
            retcode: Retcode::Succ,
            equip_list: protocol::util::build_sync_equip_info_list(&ctx.session.player_info),
        })
    }

    pub async fn on_rpc_get_resource_data_arg(
        ctx: &mut NetworkContext<'_, RpcGetResourceDataArg>,
    ) -> Result<RpcGetResourceDataRet, Retcode> {
        Ok(RpcGetResourceDataRet {
            retcode: Retcode::Succ,
            resource_list: protocol::util::build_sync_resource_info_list(&ctx.session.player_info),
            auto_recovery_info: protocol::util::build_sync_auto_recovery_info(
                &ctx.session.player_info,
            ),
        })
    }

    pub async fn on_rpc_get_avatar_data_arg(
        ctx: &mut NetworkContext<'_, RpcGetAvatarDataArg>,
    ) -> Result<RpcGetAvatarDataRet, Retcode> {
        Ok(RpcGetAvatarDataRet {
            retcode: Retcode::Succ,
            avatar_list: protocol::util::build_sync_avatar_info_list(&ctx.session.player_info),
        })
    }

    pub async fn on_rpc_get_avatar_recommend_equip_arg(
        _ctx: &mut NetworkContext<'_, RpcGetAvatarRecommendEquipArg>,
    ) -> Result<RpcGetAvatarRecommendEquipRet, Retcode> {
        Ok(RpcGetAvatarRecommendEquipRet {
            retcode: Retcode::Succ,
        })
    }

    pub async fn on_rpc_get_wishlist_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetWishlistDataArg>,
    ) -> Result<RpcGetWishlistDataRet, Retcode> {
        Ok(RpcGetWishlistDataRet {
            retcode: Retcode::Succ,
            wishlist_plan_list: Vec::with_capacity(0),
        })
    }

    pub async fn on_rpc_get_buddy_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetBuddyDataArg>,
    ) -> Result<RpcGetBuddyDataRet, Retcode> {
        Ok(RpcGetBuddyDataRet::default())
    }

    pub async fn on_rpc_weapon_dress_arg(
        ctx: &mut NetworkContext<'_, RpcWeaponDressArg>,
    ) -> Result<RpcWeaponDressRet, Retcode> {
        let Some(target_avatar_uid) = ctx
            .session
            .player_info
            .items()
            .iter()
            .find(|(_, item)| {
                if let ItemInfo::AvatarInfo { id, .. } = item {
                    *id as u32 == ctx.arg.avatar_id
                } else {
                    false
                }
            })
            .map(|(uid, _)| *uid)
        else {
            debug!("target avatar not found");
            return Err(Retcode::Fail);
        };

        let Some((_, ItemInfo::Weapon { avatar_uid, .. })) = ctx
            .session
            .player_info
            .items_mut()
            .iter_mut()
            .find(|(uid, _)| (*uid & 0xFFFFFFFF) as u32 == ctx.arg.weapon_uid)
        else {
            debug!("target weapon not found");
            return Err(Retcode::Fail);
        };

        *avatar_uid = target_avatar_uid;

        ctx.rpc_ptc
            .send_ptc(PtcPlayerSyncArg {
                avatar: Some(protocol::util::build_avatar_sync(&ctx.session.player_info)),
                item: Some(protocol::util::build_item_sync(&ctx.session.player_info)),
                ..Default::default()
            })
            .await;

        Ok(RpcWeaponDressRet::default())
    }

    pub async fn on_rpc_weapon_un_dress_arg(
        ctx: &mut NetworkContext<'_, RpcWeaponUnDressArg>,
    ) -> Result<RpcWeaponUnDressRet, Retcode> {
        let Some(target_avatar_uid) = ctx
            .session
            .player_info
            .items()
            .iter()
            .find(|(_, item)| {
                if let ItemInfo::AvatarInfo { id, .. } = item {
                    *id as u32 == ctx.arg.avatar_id
                } else {
                    false
                }
            })
            .map(|(uid, _)| *uid)
        else {
            debug!("target avatar not found");
            return Err(Retcode::Fail);
        };

        ctx.session
            .player_info
            .items_mut()
            .iter_mut()
            .for_each(|(_, item)| {
                if let ItemInfo::Weapon { avatar_uid, .. } = item {
                    if *avatar_uid == target_avatar_uid {
                        *avatar_uid = 0;
                    }
                }
            });

        ctx.rpc_ptc
            .send_ptc(PtcPlayerSyncArg {
                avatar: Some(protocol::util::build_avatar_sync(&ctx.session.player_info)),
                item: Some(protocol::util::build_item_sync(&ctx.session.player_info)),
                ..Default::default()
            })
            .await;

        Ok(RpcWeaponUnDressRet::default())
    }
}
