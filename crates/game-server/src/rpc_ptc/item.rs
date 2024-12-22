use item_info::ItemInfo;
use tracing::{debug, instrument};

use super::*;

pub async fn on_rpc_get_weapon_data_arg(
    ctx: &mut NetworkContext<'_, '_, RpcGetWeaponDataArg>,
) -> Result<RpcGetWeaponDataRet, i32> {
    Ok(RpcGetWeaponDataRet {
        retcode: 0,
        weapon_list: protocol::util::build_sync_weapon_info_list(&ctx.session.player_info),
    })
}

pub async fn on_rpc_get_equip_data_arg(
    ctx: &mut NetworkContext<'_, '_, RpcGetEquipDataArg>,
) -> Result<RpcGetEquipDataRet, i32> {
    Ok(RpcGetEquipDataRet {
        retcode: 0,
        equip_list: protocol::util::build_sync_equip_info_list(&ctx.session.player_info),
    })
}

pub async fn on_rpc_get_resource_data_arg(
    ctx: &mut NetworkContext<'_, '_, RpcGetResourceDataArg>,
) -> Result<RpcGetResourceDataRet, i32> {
    Ok(RpcGetResourceDataRet {
        retcode: 0,
        resource_list: protocol::util::build_sync_resource_info_list(&ctx.session.player_info),
        auto_recovery_info: protocol::util::build_sync_auto_recovery_info(&ctx.session.player_info),
    })
}

pub async fn on_rpc_get_avatar_data_arg(
    ctx: &mut NetworkContext<'_, '_, RpcGetAvatarDataArg>,
) -> Result<RpcGetAvatarDataRet, i32> {
    Ok(RpcGetAvatarDataRet {
        retcode: 0,
        avatar_list: protocol::util::build_sync_avatar_info_list(&ctx.session.player_info),
    })
}

pub async fn on_rpc_get_avatar_recommend_equip_arg(
    _: &mut NetworkContext<'_, '_, RpcGetAvatarRecommendEquipArg>,
) -> Result<RpcGetAvatarRecommendEquipRet, i32> {
    Ok(RpcGetAvatarRecommendEquipRet { retcode: 0 })
}

pub async fn on_rpc_get_wishlist_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetWishlistDataArg>,
) -> Result<RpcGetWishlistDataRet, i32> {
    Ok(RpcGetWishlistDataRet {
        retcode: 0,
        wishlist_plan_list: Vec::with_capacity(0),
    })
}

pub async fn on_rpc_get_buddy_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetBuddyDataArg>,
) -> Result<RpcGetBuddyDataRet, i32> {
    Ok(RpcGetBuddyDataRet::default())
}

#[instrument(skip(ctx))]
pub async fn on_rpc_weapon_dress_arg(
    ctx: &mut NetworkContext<'_, '_, RpcWeaponDressArg>,
) -> Result<RpcWeaponDressRet, i32> {
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
        return Err(-1);
    };

    let Some((_, ItemInfo::Weapon { avatar_uid, .. })) = ctx
        .session
        .player_info
        .items_mut()
        .iter_mut()
        .find(|(uid, _)| (*uid & 0xFFFFFFFF) as u32 == ctx.arg.weapon_uid)
    else {
        debug!("target weapon not found");
        return Err(-1);
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

#[instrument(skip(ctx))]
pub async fn on_rpc_weapon_un_dress_arg(
    ctx: &mut NetworkContext<'_, '_, RpcWeaponUnDressArg>,
) -> Result<RpcWeaponUnDressRet, i32> {
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
        return Err(-1);
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
