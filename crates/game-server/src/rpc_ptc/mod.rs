use qwer_rpc::{middleware::MiddlewareModel, RpcPtcContext, RpcPtcPoint};

use crate::{Globals, PlayerSession};
use paste::paste;
use protocol::*;
use qwer::*;

mod abyss;
mod activity;
mod arcade;
mod babel_tower;
mod battle_pass;
mod camp_idle;
mod daily_challenge;
mod embattles;
mod fishing_contest;
mod gacha;
mod hadal_zone;
mod interact;
mod item;
mod main_city;
mod player;
mod quest;
mod ridus_got_boo;
mod shop;
mod social;
mod unlock;
mod vhs_store;
mod world;

use abyss::*;
use activity::*;
use arcade::*;
use babel_tower::*;
use battle_pass::*;
use camp_idle::*;
use daily_challenge::*;
use embattles::*;
use fishing_contest::*;
use gacha::*;
use hadal_zone::*;
use interact::*;
use item::*;
use main_city::*;
use player::*;
use quest::*;
use ridus_got_boo::*;
use shop::*;
use social::*;
use unlock::*;
use vhs_store::*;
use world::*;

pub struct NetworkContext<'s, 'g, T> {
    pub arg: T,
    pub rpc_ptc: RpcPtcContext,
    pub session: &'s mut PlayerSession,
    #[expect(dead_code)]
    pub globals: &'g Globals,
}

macro_rules! rpc_handlers {
    (($rpc_ptc_point:ident) $($name:ident;)*) => {
        paste! {
            $(
                async fn [<_on_ $name:snake _arg>](ctx: ::qwer_rpc::RpcPtcContext) {
                    let Ok(arg) = ctx.get_arg::<::protocol::[<$name Arg>]>() else {
                        ::tracing::warn!("failed to unmarshal arg {}", stringify!($name));
                        return;
                    };

                    let Some(MiddlewareModel::Account(account_mw)) = ctx
                        .middleware_list
                        .iter()
                        .find(|&mw| matches!(mw, MiddlewareModel::Account(_)))
                    else {
                        ::tracing::warn!("failed to handle {}: account middleware is missing", stringify!($name));
                        return;
                    };

                    let Some(mut session) = crate::PLAYER_MAP.get_mut(&account_mw.player_uid) else {
                        ::tracing::warn!("failed to handle {}: player session with uid {} is not active", stringify!($name), account_mw.player_uid);
                        return;
                    };

                    let mut ctx = NetworkContext {
                        arg,
                        rpc_ptc: ctx,
                        session: &mut session,
                        globals: crate::GLOBALS.get().unwrap(),
                    };

                    match [<on_ $name:snake _arg>](&mut ctx).await {
                        Ok(ret) => {
                            ctx.rpc_ptc.send_ret(ret).await;
                            ::tracing::info!("successfully handled {}", stringify!($name));
                        },
                        Err(retcode) => {
                            ::tracing::warn!("on_{}_arg returned error code: {}", stringify!([<$name:snake>]), retcode);
                            ctx.rpc_ptc.send_ret([<$name Ret>] {
                                retcode,
                                ..Default::default()
                            }).await;
                        }
                    }

                    crate::post_rpc_handle(&mut session).await;
                }
            )*
        }

        $(
            paste!($rpc_ptc_point.register_rpc_recv(::protocol::[<$name Arg>]::PROTOCOL_ID, [<_on_ $name:snake _arg>]));
        )*
    };
}

macro_rules! ptc_handlers {
    (($rpc_ptc_point:ident) $($name:ident;)*) => {
        paste! {
            $(
                async fn [<_on_ $name:snake _arg>](ctx: ::qwer_rpc::RpcPtcContext) {
                    let Ok(arg) = ctx.get_arg::<::protocol::[<$name Arg>]>() else {
                        ::tracing::warn!("failed to unmarshal arg {}", stringify!($name));
                        return;
                    };

                    let Some(MiddlewareModel::Account(account_mw)) = ctx
                        .middleware_list
                        .iter()
                        .find(|&mw| matches!(mw, MiddlewareModel::Account(_)))
                    else {
                        ::tracing::warn!("failed to handle {}: account middleware is missing", stringify!($name));
                        return;
                    };

                    let Some(mut session) = crate::PLAYER_MAP.get_mut(&account_mw.player_uid) else {
                        ::tracing::warn!("failed to handle {}: player session with uid {} is not active", stringify!($name), account_mw.player_uid);
                        return;
                    };

                    let mut ctx = NetworkContext {
                        arg,
                        rpc_ptc: ctx,
                        session: &mut session,
                        globals: crate::GLOBALS.get().unwrap(),
                    };

                    [<on_ $name:snake _arg>](&mut ctx).await;
                    ::tracing::info!("successfully handled {}", stringify!($name));
                    crate::post_rpc_handle(&mut session).await;
                }
            )*
        }

        $(
            paste!($rpc_ptc_point.register_rpc_recv(::protocol::[<$name Arg>]::PROTOCOL_ID, [<_on_ $name:snake _arg>]));
        )*
    }
}

pub fn register_handlers(listen_point: &RpcPtcPoint) {
    rpc_handlers! {
        (listen_point)
        RpcGetPlayerBasicInfo;
        RpcGetWeaponData;
        RpcGetEquipData;
        RpcGetResourceData;
        RpcGetAvatarData;
        RpcGetWishlistData; // new 1.4
        RpcGetQuestData;
        RpcGetArchiveData;
        RpcGetHollowData;
        RpcAbyssGetData;
        RpcGetBuddyData;
        RpcAbyssArpeggioGetData;
        RpcGetServerTimestamp;
        RpcVideoGetInfo;
        RpcGetAuthkey;
        RpcGetGachaData;
        RpcGetCampIdleData;
        RpcSavePlayerSystemSetting;
        RpcGetRamenData;
        RpcGetCafeData;
        RpcGetRewardBuffData;
        RpcGetRedDotList;
        RpcGetPlayerMails;
        RpcGetFairyData;
        RpcGetTipsInfo;
        RpcGetClientSystemsData;
        RpcGetPrivateMessageData;
        RpcGetCollectMap;
        RpcWorkbenchGetData;
        RpcGetAbyssRewardData;
        RpcGetVhsStoreData;
        RpcGetActivityData;
        RpcGetWebActivityData;
        RpcGetEmbattlesData;
        RpcGetNewsStandData;
        RpcGetTrashbinHermitData;
        RpcGetMainCityRevivalData;
        RpcGetArcadeData;
        RpcGetBattlePassData;
        RpcGetHadalZoneData;
        RpcGetBabelTowerData;
        RpcGetDailyChallengeData;
        RpcGetRoleCardData;
        RpcGetChatEmojiList;
        RpcGetFriendList;
        RpcGetCharacterQuestList;
        RpcGetExplorationData;
        RpcGetFashionStoreData;
        RpcGetShoppingMallInfo;

        // new 1.4
        RpcGetMiniscapeEntrustData;
        RpcGetJourneyData;

        // new 1.5
        RpcGetRidusGotBooData;
        RpcGetFishingContestData;

        RpcGetOnlineFriendsList;
        RpcEnterWorld;
        RpcPostEnterWorld;

        RpcSceneTransition;
        RpcEnterSectionComplete;
        RpcReportEmbattleInfo;
        RpcGetMonthCardRewardList;
        RpcGetDisplayCaseData;
        RpcGetPhotoWallData;
        RpcSavePosInMainCity;
        RpcReportUiLayoutPlatform;
        RpcPlayerOperation;
        RpcGetAvatarRecommendEquip;
        RpcPlayerTransaction;
        RpcRechargeGetItemList;

        RpcModTime;
        RpcModMainCityAvatar;
        RpcInteractWithClientEntity;
        RpcInteractWithUnit;
        RpcRunEventGraph;
        RpcEnterSection;
        RpcRefreshSection;

        RpcCheckYorozuyaInfoRefresh;
        RpcBeginTrainingCourseBattle;
        RpcBattleReport;
        RpcEndBattle;
        RpcLeaveCurScene;

        RpcGetPlayerNetworkData;
        RpcWeaponDress;
        RpcWeaponUnDress;

        // Just settings stuff
        RpcSetLanguage;

        RpcSelectPostGirl;
    };

    ptc_handlers! {
        (listen_point)
        PtcKeepAlive;
    }
}
