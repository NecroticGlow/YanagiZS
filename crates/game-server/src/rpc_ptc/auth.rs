use common::time_util;
use tracing::{error, info, warn};

use crate::{level::LevelEventGraphManager, DB_CONTEXT, GLOBALS, PLAYER_MAP};

use super::*;

pub async fn on_rpc_player_login_arg(ctx: RpcPtcContext) {
    let _arg: RpcPlayerLoginArg = ctx.get_arg().unwrap();

    let Some(MiddlewareModel::Account(account_mw)) = ctx
        .middleware_list
        .iter()
        .find(|&mw| matches!(mw, MiddlewareModel::Account(_)))
    else {
        warn!("login failed: account middleware is missing");
        return;
    };

    let Ok((uid_counter, mut player_info)) = DB_CONTEXT
        .get()
        .unwrap()
        .get_or_create_player_data(GLOBALS.get().unwrap(), account_mw.player_uid)
        .await
        .inspect_err(|err| error!("login failed: get_or_create_player_data failed: {err}"))
    else {
        ctx.send_ret(RpcPlayerLoginRet {
            retcode: Retcode::Fail,
        })
        .await;
        return;
    };

    *player_info.login_times_mut() += 1;

    PLAYER_MAP.insert(
        account_mw.player_uid,
        PlayerSession {
            player_uid: account_mw.player_uid,
            uid_counter,
            player_info,
            level_event_graph_mgr: LevelEventGraphManager::new(common::util::Ptr::Static(
                GLOBALS.get().unwrap(),
            )),
            last_save_time: time_util::unix_timestamp(),
        },
    );

    info!("player with uid {} is logging in!", account_mw.player_uid);
    ctx.send_ret(RpcPlayerLoginRet {
        retcode: Retcode::Succ,
    })
    .await;
}
