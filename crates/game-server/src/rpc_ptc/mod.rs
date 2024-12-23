use common::{time_util, util::Ptr};
use qwer_rpc::{middleware::MiddlewareModel, RpcPtcContext, RpcPtcPoint};
use tracing::info;

use crate::{Globals, PlayerSession};
use protocol::*;
use qwer::*;

mod auth;

pub struct NetworkContext<'s, T: OctData> {
    pub arg: T,
    pub rpc_ptc: RpcPtcContext,
    pub session: &'s mut PlayerSession,
    pub globals: Ptr<Globals>,
}

macro_rules! network_modules {
    ($($name:ident),*) => {
        $(mod $name;)*

        fn register_module_handlers(point: &RpcPtcPoint) {
            $($name::register_handlers(point);)*
        }
    };
}

network_modules!(
    camp_idle,
    abyss,
    activity,
    arcade,
    babel_tower,
    battle_pass,
    daily_challenge,
    embattles,
    fishing_contest,
    gacha,
    hadal_zone,
    interact,
    item,
    main_city,
    perform,
    player,
    quest,
    ridus_got_boo,
    shop,
    social,
    unlock,
    vhs_store,
    world
);

pub fn register_handlers(listen_point: &RpcPtcPoint) {
    listen_point.register_rpc_recv(
        RpcPlayerLoginArg::PROTOCOL_ID,
        auth::on_rpc_player_login_arg,
    );

    register_module_handlers(listen_point);
}

pub async fn post_rpc_handle(session: &mut PlayerSession) {
    let timestamp = time_util::unix_timestamp();

    if (timestamp - session.last_save_time) >= 30 {
        session.last_save_time = timestamp;
        crate::DB_CONTEXT
            .get()
            .unwrap()
            .save_player_data(session.uid_counter.last_uid(), &session.player_info)
            .await
            .expect("failed to save player data");

        info!(
            "successfully saved player data (uid: {})",
            session.player_uid
        );
    }
}
