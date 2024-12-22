#[allow(unused)]
impl From<::protocol::RpcPlayerOperationArg> for PlayerOperationCsReq {
    fn from(value: ::protocol::RpcPlayerOperationArg) -> Self {
        Self {
            system: value.system.into(),
            param: value.param.into(),
            operator: value.operator.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<PlayerOperationCsReq> for ::protocol::RpcPlayerOperationArg {
    fn from(value: PlayerOperationCsReq) -> Self {
        Self {
            system: value.system.into(),
            param: value.param.into(),
            operator: value.operator.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetArcadeDataRet> for GetArcadeDataScRsp {
    fn from(value: ::protocol::RpcGetArcadeDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetArcadeDataScRsp> for ::protocol::RpcGetArcadeDataRet {
    fn from(value: GetArcadeDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetAuthkeyRet> for GetAuthkeyScRsp {
    fn from(value: ::protocol::RpcGetAuthkeyRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetAuthkeyScRsp> for ::protocol::RpcGetAuthkeyRet {
    fn from(value: GetAuthkeyScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::ClientSystemsSync> for ClientSystemsSync {
    fn from(value: ::protocol::ClientSystemsSync) -> Self {
        Self {
            post_girl_data: value.post_girl_data.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ClientSystemsSync> for ::protocol::ClientSystemsSync {
    fn from(value: ClientSystemsSync) -> Self {
        Self {
            post_girl_data: value.post_girl_data.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetActivityDataRet> for GetActivityDataScRsp {
    fn from(value: ::protocol::RpcGetActivityDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetActivityDataScRsp> for ::protocol::RpcGetActivityDataRet {
    fn from(value: GetActivityDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::AutoRecoveryInfo> for AutoRecoveryInfo {
    fn from(value: ::protocol::AutoRecoveryInfo) -> Self {
        Self {
            last_recovery_timestamp: value.last_recovery_timestamp.into(),
            buy_times: value.buy_times.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<AutoRecoveryInfo> for ::protocol::AutoRecoveryInfo {
    fn from(value: AutoRecoveryInfo) -> Self {
        Self {
            last_recovery_timestamp: value.last_recovery_timestamp.into(),
            buy_times: value.buy_times.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::WorkbenchData> for WorkbenchData {
    fn from(value: ::protocol::WorkbenchData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<WorkbenchData> for ::protocol::WorkbenchData {
    fn from(value: WorkbenchData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetNewsStandDataArg> for GetNewsStandDataCsReq {
    fn from(value: ::protocol::RpcGetNewsStandDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetNewsStandDataCsReq> for ::protocol::RpcGetNewsStandDataArg {
    fn from(value: GetNewsStandDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcModTimeArg> for ModTimeCsReq {
    fn from(value: ::protocol::RpcModTimeArg) -> Self {
        Self {
            time_period: value.time_period.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ModTimeCsReq> for ::protocol::RpcModTimeArg {
    fn from(value: ModTimeCsReq) -> Self {
        Self {
            time_period: value.time_period.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::DailyChallengeData> for DailyChallengeData {
    fn from(value: ::protocol::DailyChallengeData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<DailyChallengeData> for ::protocol::DailyChallengeData {
    fn from(value: DailyChallengeData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRamenDataArg> for GetRamenDataCsReq {
    fn from(value: ::protocol::RpcGetRamenDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetRamenDataCsReq> for ::protocol::RpcGetRamenDataArg {
    fn from(value: GetRamenDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetExplorationDataArg> for GetExplorationDataCsReq {
    fn from(value: ::protocol::RpcGetExplorationDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetExplorationDataCsReq> for ::protocol::RpcGetExplorationDataArg {
    fn from(value: GetExplorationDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::VideotapeInfo> for VideotapeInfo {
    fn from(value: ::protocol::VideotapeInfo) -> Self {
        Self {
            archive_file_id: value.archive_file_id.into(),
            finished: value.finished.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<VideotapeInfo> for ::protocol::VideotapeInfo {
    fn from(value: VideotapeInfo) -> Self {
        Self {
            archive_file_id: value.archive_file_id.into(),
            finished: value.finished.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcRefreshSectionArg> for RefreshSectionCsReq {
    fn from(value: ::protocol::RpcRefreshSectionArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<RefreshSectionCsReq> for ::protocol::RpcRefreshSectionArg {
    fn from(value: RefreshSectionCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetDailyChallengeDataArg> for GetDailyChallengeDataCsReq {
    fn from(value: ::protocol::RpcGetDailyChallengeDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetDailyChallengeDataCsReq> for ::protocol::RpcGetDailyChallengeDataArg {
    fn from(value: GetDailyChallengeDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::EmbattlesData> for EmbattlesData {
    fn from(value: ::protocol::EmbattlesData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<EmbattlesData> for ::protocol::EmbattlesData {
    fn from(value: EmbattlesData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetEmbattlesDataArg> for GetEmbattlesDataCsReq {
    fn from(value: ::protocol::RpcGetEmbattlesDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetEmbattlesDataCsReq> for ::protocol::RpcGetEmbattlesDataArg {
    fn from(value: GetEmbattlesDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::PostGirlData> for PostGirlData {
    fn from(value: ::protocol::PostGirlData) -> Self {
        Self {
            show_random_selected: value.show_random_selected.into(),
            post_girl_item_list: value
                .post_girl_item_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            selected_post_girl_id_list: value
                .selected_post_girl_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<PostGirlData> for ::protocol::PostGirlData {
    fn from(value: PostGirlData) -> Self {
        Self {
            show_random_selected: value.show_random_selected.into(),
            post_girl_item_list: value
                .post_girl_item_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            selected_post_girl_id_list: value
                .selected_post_girl_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::WeaponInfo> for WeaponInfo {
    fn from(value: ::protocol::WeaponInfo) -> Self {
        Self {
            level: value.level.into(),
            refine_level: value.refine_level.into(),
            id: value.id.into(),
            exp: value.exp.into(),
            star: value.star.into(),
            uid: value.uid.into(),
            lock: value.lock.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<WeaponInfo> for ::protocol::WeaponInfo {
    fn from(value: WeaponInfo) -> Self {
        Self {
            level: value.level.into(),
            refine_level: value.refine_level.into(),
            id: value.id.into(),
            exp: value.exp.into(),
            star: value.star.into(),
            uid: value.uid.into(),
            lock: value.lock.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RoleCardData> for RoleCardData {
    fn from(value: ::protocol::RoleCardData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<RoleCardData> for ::protocol::RoleCardData {
    fn from(value: RoleCardData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetCampIdleDataArg> for GetCampIdleDataCsReq {
    fn from(value: ::protocol::RpcGetCampIdleDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetCampIdleDataCsReq> for ::protocol::RpcGetCampIdleDataArg {
    fn from(value: GetCampIdleDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::EquipInfo> for EquipInfo {
    fn from(value: ::protocol::EquipInfo) -> Self {
        Self {
            star: value.star.into(),
            lock: value.lock.into(),
            uid: value.uid.into(),
            exp: value.exp.into(),
            level: value.level.into(),
            id: value.id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<EquipInfo> for ::protocol::EquipInfo {
    fn from(value: EquipInfo) -> Self {
        Self {
            star: value.star.into(),
            lock: value.lock.into(),
            uid: value.uid.into(),
            exp: value.exp.into(),
            level: value.level.into(),
            id: value.id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::DungeonInfo> for DungeonInfo {
    fn from(value: ::protocol::DungeonInfo) -> Self {
        Self {
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            dungeon_equip_info: Some(value.dungeon_equip_info.into()),
            quest_id: value.quest_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<DungeonInfo> for ::protocol::DungeonInfo {
    fn from(value: DungeonInfo) -> Self {
        Self {
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            dungeon_equip_info: value
                .dungeon_equip_info
                .map(|v| v.into())
                .unwrap_or_default(),
            quest_id: value.quest_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::MainCityRevivalData> for MainCityRevivalData {
    fn from(value: ::protocol::MainCityRevivalData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<MainCityRevivalData> for ::protocol::MainCityRevivalData {
    fn from(value: MainCityRevivalData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetClientSystemsDataRet> for GetClientSystemsDataScRsp {
    fn from(value: ::protocol::RpcGetClientSystemsDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            data: Some(value.data.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetClientSystemsDataScRsp> for ::protocol::RpcGetClientSystemsDataRet {
    fn from(value: GetClientSystemsDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            data: value.data.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::PtcKeepAliveArg> for KeepAliveNotify {
    fn from(value: ::protocol::PtcKeepAliveArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<KeepAliveNotify> for ::protocol::PtcKeepAliveArg {
    fn from(value: KeepAliveNotify) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::FairyData> for FairyData {
    fn from(value: ::protocol::FairyData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<FairyData> for ::protocol::FairyData {
    fn from(value: FairyData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetFishingContestDataRet> for GetFishingContestDataScRsp {
    fn from(value: ::protocol::RpcGetFishingContestDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetFishingContestDataScRsp> for ::protocol::RpcGetFishingContestDataRet {
    fn from(value: GetFishingContestDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetOnlineFriendsListArg> for GetOnlineFriendsListCsReq {
    fn from(value: ::protocol::RpcGetOnlineFriendsListArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetOnlineFriendsListCsReq> for ::protocol::RpcGetOnlineFriendsListArg {
    fn from(value: GetOnlineFriendsListCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRidusGotBooDataRet> for GetRidusGotBooDataScRsp {
    fn from(value: ::protocol::RpcGetRidusGotBooDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetRidusGotBooDataScRsp> for ::protocol::RpcGetRidusGotBooDataRet {
    fn from(value: GetRidusGotBooDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetExplorationDataRet> for GetExplorationDataScRsp {
    fn from(value: ::protocol::RpcGetExplorationDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetExplorationDataScRsp> for ::protocol::RpcGetExplorationDataRet {
    fn from(value: GetExplorationDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::AvatarSync> for AvatarSync {
    fn from(value: ::protocol::AvatarSync) -> Self {
        Self {
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<AvatarSync> for ::protocol::AvatarSync {
    fn from(value: AvatarSync) -> Self {
        Self {
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcEnterSectionArg> for EnterSectionCsReq {
    fn from(value: ::protocol::RpcEnterSectionArg) -> Self {
        Self {
            transform_id: value.transform_id.into(),
            section_id: value.section_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<EnterSectionCsReq> for ::protocol::RpcEnterSectionArg {
    fn from(value: EnterSectionCsReq) -> Self {
        Self {
            transform_id: value.transform_id.into(),
            section_id: value.section_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::PostGirlItem> for PostGirlItem {
    fn from(value: ::protocol::PostGirlItem) -> Self {
        Self {
            unlock_time: value.unlock_time.into(),
            id: value.id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<PostGirlItem> for ::protocol::PostGirlItem {
    fn from(value: PostGirlItem) -> Self {
        Self {
            unlock_time: value.unlock_time.into(),
            id: value.id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::LevelRewardInfo> for LevelRewardInfo {
    fn from(value: ::protocol::LevelRewardInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<LevelRewardInfo> for ::protocol::LevelRewardInfo {
    fn from(value: LevelRewardInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetJourneyDataRet> for GetJourneyDataScRsp {
    fn from(value: ::protocol::RpcGetJourneyDataRet) -> Self {
        Self {
            data: Some(value.data.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetJourneyDataScRsp> for ::protocol::RpcGetJourneyDataRet {
    fn from(value: GetJourneyDataScRsp) -> Self {
        Self {
            data: value.data.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetCafeDataArg> for GetCafeDataCsReq {
    fn from(value: ::protocol::RpcGetCafeDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetCafeDataCsReq> for ::protocol::RpcGetCafeDataArg {
    fn from(value: GetCafeDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::JourneyData> for JourneyData {
    fn from(value: ::protocol::JourneyData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<JourneyData> for ::protocol::JourneyData {
    fn from(value: JourneyData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPlayerMailsArg> for GetPlayerMailsCsReq {
    fn from(value: ::protocol::RpcGetPlayerMailsArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetPlayerMailsCsReq> for ::protocol::RpcGetPlayerMailsArg {
    fn from(value: GetPlayerMailsCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetMiniscapeEntrustDataRet> for GetMiniscapeEntrustDataScRsp {
    fn from(value: ::protocol::RpcGetMiniscapeEntrustDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetMiniscapeEntrustDataScRsp> for ::protocol::RpcGetMiniscapeEntrustDataRet {
    fn from(value: GetMiniscapeEntrustDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRamenDataRet> for GetRamenDataScRsp {
    fn from(value: ::protocol::RpcGetRamenDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ramen_data: Some(value.ramen_data.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetRamenDataScRsp> for ::protocol::RpcGetRamenDataRet {
    fn from(value: GetRamenDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ramen_data: value.ramen_data.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRoleCardDataArg> for GetRoleCardDataCsReq {
    fn from(value: ::protocol::RpcGetRoleCardDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetRoleCardDataCsReq> for ::protocol::RpcGetRoleCardDataArg {
    fn from(value: GetRoleCardDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetArchiveDataRet> for GetArchiveDataScRsp {
    fn from(value: ::protocol::RpcGetArchiveDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            archive_data: Some(value.archive_data.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetArchiveDataScRsp> for ::protocol::RpcGetArchiveDataRet {
    fn from(value: GetArchiveDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            archive_data: value.archive_data.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetMainCityRevivalDataRet> for GetMainCityRevivalDataScRsp {
    fn from(value: ::protocol::RpcGetMainCityRevivalDataRet) -> Self {
        Self {
            main_city_revival_data: Some(value.main_city_revival_data.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetMainCityRevivalDataScRsp> for ::protocol::RpcGetMainCityRevivalDataRet {
    fn from(value: GetMainCityRevivalDataScRsp) -> Self {
        Self {
            main_city_revival_data: value
                .main_city_revival_data
                .map(|v| v.into())
                .unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetCollectMapRet> for GetCollectMapScRsp {
    fn from(value: ::protocol::RpcGetCollectMapRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            collect_map: Some(value.collect_map.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetCollectMapScRsp> for ::protocol::RpcGetCollectMapRet {
    fn from(value: GetCollectMapScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            collect_map: value.collect_map.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPrivateMessageDataRet> for GetPrivateMessageDataScRsp {
    fn from(value: ::protocol::RpcGetPrivateMessageDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetPrivateMessageDataScRsp> for ::protocol::RpcGetPrivateMessageDataRet {
    fn from(value: GetPrivateMessageDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetGachaDataArg> for GetGachaDataCsReq {
    fn from(value: ::protocol::RpcGetGachaDataArg) -> Self {
        Self {
            gacha_type: value.gacha_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetGachaDataCsReq> for ::protocol::RpcGetGachaDataArg {
    fn from(value: GetGachaDataCsReq) -> Self {
        Self {
            gacha_type: value.gacha_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetActivityDataArg> for GetActivityDataCsReq {
    fn from(value: ::protocol::RpcGetActivityDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetActivityDataCsReq> for ::protocol::RpcGetActivityDataArg {
    fn from(value: GetActivityDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::CampIdleData> for CampIdleData {
    fn from(value: ::protocol::CampIdleData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<CampIdleData> for ::protocol::CampIdleData {
    fn from(value: CampIdleData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcEnterSectionCompleteRet> for EnterSectionCompleteScRsp {
    fn from(value: ::protocol::RpcEnterSectionCompleteRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<EnterSectionCompleteScRsp> for ::protocol::RpcEnterSectionCompleteRet {
    fn from(value: EnterSectionCompleteScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPlayerNetworkDataArg> for GetPlayerNetworkDataCsReq {
    fn from(value: ::protocol::RpcGetPlayerNetworkDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetPlayerNetworkDataCsReq> for ::protocol::RpcGetPlayerNetworkDataArg {
    fn from(value: GetPlayerNetworkDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::DungeonEquipInfo> for DungeonEquipInfo {
    fn from(value: ::protocol::DungeonEquipInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<DungeonEquipInfo> for ::protocol::DungeonEquipInfo {
    fn from(value: DungeonEquipInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetNewsStandDataRet> for GetNewsStandDataScRsp {
    fn from(value: ::protocol::RpcGetNewsStandDataRet) -> Self {
        Self {
            news_stand_data: Some(value.news_stand_data.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetNewsStandDataScRsp> for ::protocol::RpcGetNewsStandDataRet {
    fn from(value: GetNewsStandDataScRsp) -> Self {
        Self {
            news_stand_data: value.news_stand_data.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetEquipDataRet> for GetEquipDataScRsp {
    fn from(value: ::protocol::RpcGetEquipDataRet) -> Self {
        Self {
            equip_list: value.equip_list.into_iter().map(|v| v.into()).collect(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetEquipDataScRsp> for ::protocol::RpcGetEquipDataRet {
    fn from(value: GetEquipDataScRsp) -> Self {
        Self {
            equip_list: value.equip_list.into_iter().map(|v| v.into()).collect(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetWebActivityDataArg> for GetWebActivityDataCsReq {
    fn from(value: ::protocol::RpcGetWebActivityDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetWebActivityDataCsReq> for ::protocol::RpcGetWebActivityDataArg {
    fn from(value: GetWebActivityDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::LevelPerformInfo> for LevelPerformInfo {
    fn from(value: ::protocol::LevelPerformInfo) -> Self {
        Self {
            time: value.time.into(),
            weather: value.weather.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<LevelPerformInfo> for ::protocol::LevelPerformInfo {
    fn from(value: LevelPerformInfo) -> Self {
        Self {
            time: value.time.into(),
            weather: value.weather.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetAbyssRewardDataArg> for GetAbyssRewardDataCsReq {
    fn from(value: ::protocol::RpcGetAbyssRewardDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetAbyssRewardDataCsReq> for ::protocol::RpcGetAbyssRewardDataArg {
    fn from(value: GetAbyssRewardDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcReportEmbattleInfoArg> for ReportEmbattleInfoCsReq {
    fn from(value: ::protocol::RpcReportEmbattleInfoArg) -> Self {
        Self {
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ReportEmbattleInfoCsReq> for ::protocol::RpcReportEmbattleInfoArg {
    fn from(value: ReportEmbattleInfoCsReq) -> Self {
        Self {
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPhotoWallDataArg> for GetPhotoWallDataCsReq {
    fn from(value: ::protocol::RpcGetPhotoWallDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetPhotoWallDataCsReq> for ::protocol::RpcGetPhotoWallDataArg {
    fn from(value: GetPhotoWallDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcPostEnterWorldArg> for PostEnterWorldCsReq {
    fn from(value: ::protocol::RpcPostEnterWorldArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<PostEnterWorldCsReq> for ::protocol::RpcPostEnterWorldArg {
    fn from(value: PostEnterWorldCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcWeaponUnDressArg> for WeaponUnDressCsReq {
    fn from(value: ::protocol::RpcWeaponUnDressArg) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<WeaponUnDressCsReq> for ::protocol::RpcWeaponUnDressArg {
    fn from(value: WeaponUnDressCsReq) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetMiniscapeEntrustDataArg> for GetMiniscapeEntrustDataCsReq {
    fn from(value: ::protocol::RpcGetMiniscapeEntrustDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetMiniscapeEntrustDataCsReq> for ::protocol::RpcGetMiniscapeEntrustDataArg {
    fn from(value: GetMiniscapeEntrustDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetAuthkeyArg> for GetAuthkeyCsReq {
    fn from(value: ::protocol::RpcGetAuthkeyArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetAuthkeyCsReq> for ::protocol::RpcGetAuthkeyArg {
    fn from(value: GetAuthkeyCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::ClientSystemsData> for ClientSystemsData {
    fn from(value: ::protocol::ClientSystemsData) -> Self {
        Self {
            post_girl_data: Some(value.post_girl_data.into()),
            unlock_data: Some(value.unlock_data.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ClientSystemsData> for ::protocol::ClientSystemsData {
    fn from(value: ClientSystemsData) -> Self {
        Self {
            post_girl_data: value.post_girl_data.map(|v| v.into()).unwrap_or_default(),
            unlock_data: value.unlock_data.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetEmbattlesDataRet> for GetEmbattlesDataScRsp {
    fn from(value: ::protocol::RpcGetEmbattlesDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetEmbattlesDataScRsp> for ::protocol::RpcGetEmbattlesDataRet {
    fn from(value: GetEmbattlesDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetQuestDataArg> for GetQuestDataCsReq {
    fn from(value: ::protocol::RpcGetQuestDataArg) -> Self {
        Self {
            quest_type: value.quest_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetQuestDataCsReq> for ::protocol::RpcGetQuestDataArg {
    fn from(value: GetQuestDataCsReq) -> Self {
        Self {
            quest_type: value.quest_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetTipsInfoArg> for GetTipsInfoCsReq {
    fn from(value: ::protocol::RpcGetTipsInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetTipsInfoCsReq> for ::protocol::RpcGetTipsInfoArg {
    fn from(value: GetTipsInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetTipsInfoRet> for GetTipsInfoScRsp {
    fn from(value: ::protocol::RpcGetTipsInfoRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetTipsInfoScRsp> for ::protocol::RpcGetTipsInfoRet {
    fn from(value: GetTipsInfoScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetWishlistDataArg> for GetWishlistDataCsReq {
    fn from(value: ::protocol::RpcGetWishlistDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetWishlistDataCsReq> for ::protocol::RpcGetWishlistDataArg {
    fn from(value: GetWishlistDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcEndBattleRet> for EndBattleScRsp {
    fn from(value: ::protocol::RpcEndBattleRet) -> Self {
        Self {
            fight_settle: Some(value.fight_settle.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<EndBattleScRsp> for ::protocol::RpcEndBattleRet {
    fn from(value: EndBattleScRsp) -> Self {
        Self {
            fight_settle: value.fight_settle.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetClientSystemsDataArg> for GetClientSystemsDataCsReq {
    fn from(value: ::protocol::RpcGetClientSystemsDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetClientSystemsDataCsReq> for ::protocol::RpcGetClientSystemsDataArg {
    fn from(value: GetClientSystemsDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetBabelTowerDataArg> for GetBabelTowerDataCsReq {
    fn from(value: ::protocol::RpcGetBabelTowerDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetBabelTowerDataCsReq> for ::protocol::RpcGetBabelTowerDataArg {
    fn from(value: GetBabelTowerDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetVhsStoreDataArg> for GetVhsStoreDataCsReq {
    fn from(value: ::protocol::RpcGetVhsStoreDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetVhsStoreDataCsReq> for ::protocol::RpcGetVhsStoreDataArg {
    fn from(value: GetVhsStoreDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetServerTimestampArg> for GetServerTimestampCsReq {
    fn from(value: ::protocol::RpcGetServerTimestampArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetServerTimestampCsReq> for ::protocol::RpcGetServerTimestampArg {
    fn from(value: GetServerTimestampCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::CafeData> for CafeData {
    fn from(value: ::protocol::CafeData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<CafeData> for ::protocol::CafeData {
    fn from(value: CafeData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetFairyDataRet> for GetFairyDataScRsp {
    fn from(value: ::protocol::RpcGetFairyDataRet) -> Self {
        Self {
            data: Some(value.data.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetFairyDataScRsp> for ::protocol::RpcGetFairyDataRet {
    fn from(value: GetFairyDataScRsp) -> Self {
        Self {
            data: value.data.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::TrashbinHermitData> for TrashbinHermitData {
    fn from(value: ::protocol::TrashbinHermitData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<TrashbinHermitData> for ::protocol::TrashbinHermitData {
    fn from(value: TrashbinHermitData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcWorkbenchGetDataRet> for WorkbenchGetDataScRsp {
    fn from(value: ::protocol::RpcWorkbenchGetDataRet) -> Self {
        Self {
            workbench_data: Some(value.workbench_data.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<WorkbenchGetDataScRsp> for ::protocol::RpcWorkbenchGetDataRet {
    fn from(value: WorkbenchGetDataScRsp) -> Self {
        Self {
            workbench_data: value.workbench_data.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcEnterWorldRet> for EnterWorldScRsp {
    fn from(value: ::protocol::RpcEnterWorldRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<EnterWorldScRsp> for ::protocol::RpcEnterWorldRet {
    fn from(value: EnterWorldScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcSelectPostGirlArg> for SelectPostGirlCsReq {
    fn from(value: ::protocol::RpcSelectPostGirlArg) -> Self {
        Self {
            new_selected_post_girl_id_list: value
                .new_selected_post_girl_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            show_random_selected: value.show_random_selected.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<SelectPostGirlCsReq> for ::protocol::RpcSelectPostGirlArg {
    fn from(value: SelectPostGirlCsReq) -> Self {
        Self {
            new_selected_post_girl_id_list: value
                .new_selected_post_girl_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            show_random_selected: value.show_random_selected.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRedDotListArg> for GetRedDotListCsReq {
    fn from(value: ::protocol::RpcGetRedDotListArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetRedDotListCsReq> for ::protocol::RpcGetRedDotListArg {
    fn from(value: GetRedDotListCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRewardBuffDataArg> for GetRewardBuffDataCsReq {
    fn from(value: ::protocol::RpcGetRewardBuffDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetRewardBuffDataCsReq> for ::protocol::RpcGetRewardBuffDataArg {
    fn from(value: GetRewardBuffDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::PlayerBasicInfo> for PlayerBasicInfo {
    fn from(value: ::protocol::PlayerBasicInfo) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            last_enter_world_timestamp: value.last_enter_world_timestamp.into(),
            player_avatar_id: value.player_avatar_id.into(),
            nick_name: value.nick_name.into(),
            level: value.level.into(),
            main_city_avatar_id: value.main_city_avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<PlayerBasicInfo> for ::protocol::PlayerBasicInfo {
    fn from(value: PlayerBasicInfo) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            last_enter_world_timestamp: value.last_enter_world_timestamp.into(),
            player_avatar_id: value.player_avatar_id.into(),
            nick_name: value.nick_name.into(),
            level: value.level.into(),
            main_city_avatar_id: value.main_city_avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::PtcHallRefreshArg> for HallRefreshScNotify {
    fn from(value: ::protocol::PtcHallRefreshArg) -> Self {
        Self {
            scene_unit_list: value
                .scene_unit_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            bgm_id: value.bgm_id.into(),
            main_city_avatar_id: value.main_city_avatar_id.into(),
            time_of_day: value.time_of_day.into(),
            player_avatar_id: value.player_avatar_id.into(),
            refresh_immediately: value.refresh_immediately.into(),
            main_city_objects_state: value
                .main_city_objects_state
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            day_of_week: value.day_of_week.into(),
            section_id: value.section_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<HallRefreshScNotify> for ::protocol::PtcHallRefreshArg {
    fn from(value: HallRefreshScNotify) -> Self {
        Self {
            scene_unit_list: value
                .scene_unit_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            bgm_id: value.bgm_id.into(),
            main_city_avatar_id: value.main_city_avatar_id.into(),
            time_of_day: value.time_of_day.into(),
            player_avatar_id: value.player_avatar_id.into(),
            refresh_immediately: value.refresh_immediately.into(),
            main_city_objects_state: value
                .main_city_objects_state
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            day_of_week: value.day_of_week.into(),
            section_id: value.section_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::CollectMap> for CollectMap {
    fn from(value: ::protocol::CollectMap) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<CollectMap> for ::protocol::CollectMap {
    fn from(value: CollectMap) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::FightSettle> for FightSettle {
    fn from(value: ::protocol::FightSettle) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<FightSettle> for ::protocol::FightSettle {
    fn from(value: FightSettle) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetBattlePassDataRet> for GetBattlePassDataScRsp {
    fn from(value: ::protocol::RpcGetBattlePassDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetBattlePassDataScRsp> for ::protocol::RpcGetBattlePassDataRet {
    fn from(value: GetBattlePassDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::ShoppingMallInfo> for ShoppingMallInfo {
    fn from(value: ::protocol::ShoppingMallInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<ShoppingMallInfo> for ::protocol::ShoppingMallInfo {
    fn from(value: ShoppingMallInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetBuddyDataArg> for GetBuddyDataCsReq {
    fn from(value: ::protocol::RpcGetBuddyDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetBuddyDataCsReq> for ::protocol::RpcGetBuddyDataArg {
    fn from(value: GetBuddyDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::PtcUpdateEventGraphArg> for UpdateEventGraphScNotify {
    fn from(value: ::protocol::PtcUpdateEventGraphArg) -> Self {
        Self {
            tag: value.tag.into(),
            event_graph_owner_uid: value.event_graph_owner_uid.into(),
            npc_interaction: value.npc_interaction.into(),
            event_graph_uid: value.event_graph_uid.into(),
            is_event_success: value.is_event_success.into(),
            owner_type: value.owner_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<UpdateEventGraphScNotify> for ::protocol::PtcUpdateEventGraphArg {
    fn from(value: UpdateEventGraphScNotify) -> Self {
        Self {
            tag: value.tag.into(),
            event_graph_owner_uid: value.event_graph_owner_uid.into(),
            npc_interaction: value.npc_interaction.into(),
            event_graph_uid: value.event_graph_uid.into(),
            is_event_success: value.is_event_success.into(),
            owner_type: value.owner_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetMainCityRevivalDataArg> for GetMainCityRevivalDataCsReq {
    fn from(value: ::protocol::RpcGetMainCityRevivalDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetMainCityRevivalDataCsReq> for ::protocol::RpcGetMainCityRevivalDataArg {
    fn from(value: GetMainCityRevivalDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RamenData> for RamenData {
    fn from(value: ::protocol::RamenData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<RamenData> for ::protocol::RamenData {
    fn from(value: RamenData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetArchiveDataArg> for GetArchiveDataCsReq {
    fn from(value: ::protocol::RpcGetArchiveDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetArchiveDataCsReq> for ::protocol::RpcGetArchiveDataArg {
    fn from(value: GetArchiveDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetShoppingMallInfoRet> for GetShoppingMallInfoScRsp {
    fn from(value: ::protocol::RpcGetShoppingMallInfoRet) -> Self {
        Self {
            shopping_mall_info: Some(value.shopping_mall_info.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetShoppingMallInfoScRsp> for ::protocol::RpcGetShoppingMallInfoRet {
    fn from(value: GetShoppingMallInfoScRsp) -> Self {
        Self {
            shopping_mall_info: value
                .shopping_mall_info
                .map(|v| v.into())
                .unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetArcadeDataArg> for GetArcadeDataCsReq {
    fn from(value: ::protocol::RpcGetArcadeDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetArcadeDataCsReq> for ::protocol::RpcGetArcadeDataArg {
    fn from(value: GetArcadeDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcVideoGetInfoArg> for VideoGetInfoCsReq {
    fn from(value: ::protocol::RpcVideoGetInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<VideoGetInfoCsReq> for ::protocol::RpcVideoGetInfoArg {
    fn from(value: VideoGetInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::HollowData> for HollowData {
    fn from(value: ::protocol::HollowData) -> Self {
        Self {
            unlock_hollow_id_list: value
                .unlock_hollow_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<HollowData> for ::protocol::HollowData {
    fn from(value: HollowData) -> Self {
        Self {
            unlock_hollow_id_list: value
                .unlock_hollow_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcCheckYorozuyaInfoRefreshArg> for CheckYorozuyaInfoRefreshCsReq {
    fn from(value: ::protocol::RpcCheckYorozuyaInfoRefreshArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<CheckYorozuyaInfoRefreshCsReq> for ::protocol::RpcCheckYorozuyaInfoRefreshArg {
    fn from(value: CheckYorozuyaInfoRefreshCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcInteractWithUnitArg> for InteractWithUnitCsReq {
    fn from(value: ::protocol::RpcInteractWithUnitArg) -> Self {
        Self {
            interaction: value.interaction.into(),
            r#type: value.r#type.into(),
            npc_tag_id: value.npc_tag_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<InteractWithUnitCsReq> for ::protocol::RpcInteractWithUnitArg {
    fn from(value: InteractWithUnitCsReq) -> Self {
        Self {
            interaction: value.interaction.into(),
            r#type: value.r#type.into(),
            npc_tag_id: value.npc_tag_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetDailyChallengeDataRet> for GetDailyChallengeDataScRsp {
    fn from(value: ::protocol::RpcGetDailyChallengeDataRet) -> Self {
        Self {
            data: Some(value.data.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetDailyChallengeDataScRsp> for ::protocol::RpcGetDailyChallengeDataRet {
    fn from(value: GetDailyChallengeDataScRsp) -> Self {
        Self {
            data: value.data.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::InteractInfo> for InteractInfo {
    fn from(value: ::protocol::InteractInfo) -> Self {
        Self {
            interact_target_list: value
                .interact_target_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            name: value.name.into(),
            interact_id: value.interact_id.into(),
            participators: value
                .participators
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<InteractInfo> for ::protocol::InteractInfo {
    fn from(value: InteractInfo) -> Self {
        Self {
            interact_target_list: value
                .interact_target_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            name: value.name.into(),
            interact_id: value.interact_id.into(),
            participators: value
                .participators
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetAvatarDataRet> for GetAvatarDataScRsp {
    fn from(value: ::protocol::RpcGetAvatarDataRet) -> Self {
        Self {
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetAvatarDataScRsp> for ::protocol::RpcGetAvatarDataRet {
    fn from(value: GetAvatarDataScRsp) -> Self {
        Self {
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::VhsStoreData> for VhsStoreData {
    fn from(value: ::protocol::VhsStoreData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<VhsStoreData> for ::protocol::VhsStoreData {
    fn from(value: VhsStoreData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetFishingContestDataArg> for GetFishingContestDataCsReq {
    fn from(value: ::protocol::RpcGetFishingContestDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetFishingContestDataCsReq> for ::protocol::RpcGetFishingContestDataArg {
    fn from(value: GetFishingContestDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetEquipDataArg> for GetEquipDataCsReq {
    fn from(value: ::protocol::RpcGetEquipDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetEquipDataCsReq> for ::protocol::RpcGetEquipDataArg {
    fn from(value: GetEquipDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcRunEventGraphRet> for RunEventGraphScRsp {
    fn from(value: ::protocol::RpcRunEventGraphRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<RunEventGraphScRsp> for ::protocol::RpcRunEventGraphRet {
    fn from(value: RunEventGraphScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPlayerBasicInfoArg> for GetPlayerBasicInfoCsReq {
    fn from(value: ::protocol::RpcGetPlayerBasicInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetPlayerBasicInfoCsReq> for ::protocol::RpcGetPlayerBasicInfoArg {
    fn from(value: GetPlayerBasicInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcPlayerLoginRet> for PlayerLoginScRsp {
    fn from(value: ::protocol::RpcPlayerLoginRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<PlayerLoginScRsp> for ::protocol::RpcPlayerLoginRet {
    fn from(value: PlayerLoginScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetResourceDataArg> for GetResourceDataCsReq {
    fn from(value: ::protocol::RpcGetResourceDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetResourceDataCsReq> for ::protocol::RpcGetResourceDataArg {
    fn from(value: GetResourceDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcEnterWorldArg> for EnterWorldCsReq {
    fn from(value: ::protocol::RpcEnterWorldArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<EnterWorldCsReq> for ::protocol::RpcEnterWorldArg {
    fn from(value: EnterWorldCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetTrashbinHermitDataRet> for GetTrashbinHermitDataScRsp {
    fn from(value: ::protocol::RpcGetTrashbinHermitDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            trashbin_hermit_data: Some(value.trashbin_hermit_data.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetTrashbinHermitDataScRsp> for ::protocol::RpcGetTrashbinHermitDataRet {
    fn from(value: GetTrashbinHermitDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            trashbin_hermit_data: value
                .trashbin_hermit_data
                .map(|v| v.into())
                .unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcBattleReportArg> for BattleReportCsReq {
    fn from(value: ::protocol::RpcBattleReportArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<BattleReportCsReq> for ::protocol::RpcBattleReportArg {
    fn from(value: BattleReportCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetCharacterQuestListArg> for GetCharacterQuestListCsReq {
    fn from(value: ::protocol::RpcGetCharacterQuestListArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetCharacterQuestListCsReq> for ::protocol::RpcGetCharacterQuestListArg {
    fn from(value: GetCharacterQuestListCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcEnterSectionCompleteArg> for EnterSectionCompleteCsReq {
    fn from(value: ::protocol::RpcEnterSectionCompleteArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<EnterSectionCompleteCsReq> for ::protocol::RpcEnterSectionCompleteArg {
    fn from(value: EnterSectionCompleteCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::QuestData> for QuestData {
    fn from(value: ::protocol::QuestData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<QuestData> for ::protocol::QuestData {
    fn from(value: QuestData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetChatEmojiListArg> for GetChatEmojiListCsReq {
    fn from(value: ::protocol::RpcGetChatEmojiListArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetChatEmojiListCsReq> for ::protocol::RpcGetChatEmojiListArg {
    fn from(value: GetChatEmojiListCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::AvatarUnitInfo> for AvatarUnitInfo {
    fn from(value: ::protocol::AvatarUnitInfo) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<AvatarUnitInfo> for ::protocol::AvatarUnitInfo {
    fn from(value: AvatarUnitInfo) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcAbyssArpeggioGetDataArg> for AbyssArpeggioGetDataCsReq {
    fn from(value: ::protocol::RpcAbyssArpeggioGetDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<AbyssArpeggioGetDataCsReq> for ::protocol::RpcAbyssArpeggioGetDataArg {
    fn from(value: AbyssArpeggioGetDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetCharacterQuestListRet> for GetCharacterQuestListScRsp {
    fn from(value: ::protocol::RpcGetCharacterQuestListRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetCharacterQuestListScRsp> for ::protocol::RpcGetCharacterQuestListRet {
    fn from(value: GetCharacterQuestListScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::NewsStandData> for NewsStandData {
    fn from(value: ::protocol::NewsStandData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<NewsStandData> for ::protocol::NewsStandData {
    fn from(value: NewsStandData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPrivateMessageDataArg> for GetPrivateMessageDataCsReq {
    fn from(value: ::protocol::RpcGetPrivateMessageDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetPrivateMessageDataCsReq> for ::protocol::RpcGetPrivateMessageDataArg {
    fn from(value: GetPrivateMessageDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::AvatarInfo> for AvatarInfo {
    fn from(value: ::protocol::AvatarInfo) -> Self {
        Self {
            unlocked_talent_num: value.unlocked_talent_num.into(),
            cur_weapon_uid: value.cur_weapon_uid.into(),
            skill_type_level: value
                .skill_type_level
                .into_iter()
                .map(|v| v.into())
                .collect(),
            id: value.id.into(),
            talent_switch_list: value
                .talent_switch_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            exp: value.exp.into(),
            level: value.level.into(),
            rank: value.rank.into(),
            first_get_time: value.first_get_time.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<AvatarInfo> for ::protocol::AvatarInfo {
    fn from(value: AvatarInfo) -> Self {
        Self {
            unlocked_talent_num: value.unlocked_talent_num.into(),
            cur_weapon_uid: value.cur_weapon_uid.into(),
            skill_type_level: value
                .skill_type_level
                .into_iter()
                .map(|v| v.into())
                .collect(),
            id: value.id.into(),
            talent_switch_list: value
                .talent_switch_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            exp: value.exp.into(),
            level: value.level.into(),
            rank: value.rank.into(),
            first_get_time: value.first_get_time.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::HallSceneInfo> for HallSceneInfo {
    fn from(value: ::protocol::HallSceneInfo) -> Self {
        Self {
            position: value.position.map(|v| v.into()),
            day_of_week: value.day_of_week.into(),
            bgm_id: value.bgm_id.into(),
            camera_x: value.camera_x.into(),
            main_city_objects_state: value
                .main_city_objects_state
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            player_avatar_id: value.player_avatar_id.into(),
            section_id: value.section_id.into(),
            time_of_day: value.time_of_day.into(),
            transform_id: value.transform_id.into(),
            camera_y: value.camera_y.into(),
            main_city_avatar_id: value.main_city_avatar_id.into(),
            scene_unit_list: value
                .scene_unit_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<HallSceneInfo> for ::protocol::HallSceneInfo {
    fn from(value: HallSceneInfo) -> Self {
        Self {
            position: value.position.map(|v| v.into()),
            day_of_week: value.day_of_week.into(),
            bgm_id: value.bgm_id.into(),
            camera_x: value.camera_x.into(),
            main_city_objects_state: value
                .main_city_objects_state
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            player_avatar_id: value.player_avatar_id.into(),
            section_id: value.section_id.into(),
            time_of_day: value.time_of_day.into(),
            transform_id: value.transform_id.into(),
            camera_y: value.camera_y.into(),
            main_city_avatar_id: value.main_city_avatar_id.into(),
            scene_unit_list: value
                .scene_unit_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRewardBuffDataRet> for GetRewardBuffDataScRsp {
    fn from(value: ::protocol::RpcGetRewardBuffDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            data: Some(value.data.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetRewardBuffDataScRsp> for ::protocol::RpcGetRewardBuffDataRet {
    fn from(value: GetRewardBuffDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            data: value.data.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::AvatarSkillInfo> for AvatarSkillInfo {
    fn from(value: ::protocol::AvatarSkillInfo) -> Self {
        Self {
            skill_type: value.skill_type.into(),
            level: value.level.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<AvatarSkillInfo> for ::protocol::AvatarSkillInfo {
    fn from(value: AvatarSkillInfo) -> Self {
        Self {
            skill_type: value.skill_type.into(),
            level: value.level.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcPlayerTransactionArg> for PlayerTransactionCsReq {
    fn from(value: ::protocol::RpcPlayerTransactionArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<PlayerTransactionCsReq> for ::protocol::RpcPlayerTransactionArg {
    fn from(value: PlayerTransactionCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetFashionStoreDataRet> for GetFashionStoreDataScRsp {
    fn from(value: ::protocol::RpcGetFashionStoreDataRet) -> Self {
        Self {
            data: Some(value.data.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetFashionStoreDataScRsp> for ::protocol::RpcGetFashionStoreDataRet {
    fn from(value: GetFashionStoreDataScRsp) -> Self {
        Self {
            data: value.data.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetAvatarRecommendEquipArg> for GetAvatarRecommendEquipCsReq {
    fn from(value: ::protocol::RpcGetAvatarRecommendEquipArg) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetAvatarRecommendEquipCsReq> for ::protocol::RpcGetAvatarRecommendEquipArg {
    fn from(value: GetAvatarRecommendEquipCsReq) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcInteractWithClientEntityArg> for InteractWithClientEntityCsReq {
    fn from(value: ::protocol::RpcInteractWithClientEntityArg) -> Self {
        Self {
            interaction: value.interaction.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<InteractWithClientEntityCsReq> for ::protocol::RpcInteractWithClientEntityArg {
    fn from(value: InteractWithClientEntityCsReq) -> Self {
        Self {
            interaction: value.interaction.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcReportUiLayoutPlatformArg> for ReportUiLayoutPlatformCsReq {
    fn from(value: ::protocol::RpcReportUiLayoutPlatformArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<ReportUiLayoutPlatformCsReq> for ::protocol::RpcReportUiLayoutPlatformArg {
    fn from(value: ReportUiLayoutPlatformCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcAbyssGetDataArg> for AbyssGetDataCsReq {
    fn from(value: ::protocol::RpcAbyssGetDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<AbyssGetDataCsReq> for ::protocol::RpcAbyssGetDataArg {
    fn from(value: AbyssGetDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetJourneyDataArg> for GetJourneyDataCsReq {
    fn from(value: ::protocol::RpcGetJourneyDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetJourneyDataCsReq> for ::protocol::RpcGetJourneyDataArg {
    fn from(value: GetJourneyDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetBattlePassDataArg> for GetBattlePassDataCsReq {
    fn from(value: ::protocol::RpcGetBattlePassDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetBattlePassDataCsReq> for ::protocol::RpcGetBattlePassDataArg {
    fn from(value: GetBattlePassDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetFriendListRet> for GetFriendListScRsp {
    fn from(value: ::protocol::RpcGetFriendListRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetFriendListScRsp> for ::protocol::RpcGetFriendListRet {
    fn from(value: GetFriendListScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetQuestDataRet> for GetQuestDataScRsp {
    fn from(value: ::protocol::RpcGetQuestDataRet) -> Self {
        Self {
            quest_type: value.quest_type.into(),
            quest_data: Some(value.quest_data.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetQuestDataScRsp> for ::protocol::RpcGetQuestDataRet {
    fn from(value: GetQuestDataScRsp) -> Self {
        Self {
            quest_type: value.quest_type.into(),
            quest_data: value.quest_data.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcRechargeGetItemListRet> for RechargeGetItemListScRsp {
    fn from(value: ::protocol::RpcRechargeGetItemListRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<RechargeGetItemListScRsp> for ::protocol::RpcRechargeGetItemListRet {
    fn from(value: RechargeGetItemListScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetWeaponDataArg> for GetWeaponDataCsReq {
    fn from(value: ::protocol::RpcGetWeaponDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetWeaponDataCsReq> for ::protocol::RpcGetWeaponDataArg {
    fn from(value: GetWeaponDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetDisplayCaseDataArg> for GetDisplayCaseDataCsReq {
    fn from(value: ::protocol::RpcGetDisplayCaseDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetDisplayCaseDataCsReq> for ::protocol::RpcGetDisplayCaseDataArg {
    fn from(value: GetDisplayCaseDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetFashionStoreDataArg> for GetFashionStoreDataCsReq {
    fn from(value: ::protocol::RpcGetFashionStoreDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetFashionStoreDataCsReq> for ::protocol::RpcGetFashionStoreDataArg {
    fn from(value: GetFashionStoreDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcVideoGetInfoRet> for VideoGetInfoScRsp {
    fn from(value: ::protocol::RpcVideoGetInfoRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<VideoGetInfoScRsp> for ::protocol::RpcVideoGetInfoRet {
    fn from(value: VideoGetInfoScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetHollowDataRet> for GetHollowDataScRsp {
    fn from(value: ::protocol::RpcGetHollowDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            hollow_data: Some(value.hollow_data.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetHollowDataScRsp> for ::protocol::RpcGetHollowDataRet {
    fn from(value: GetHollowDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            hollow_data: value.hollow_data.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcSceneTransitionArg> for SceneTransitionCsReq {
    fn from(value: ::protocol::RpcSceneTransitionArg) -> Self {
        Self {
            section_id: value.section_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<SceneTransitionCsReq> for ::protocol::RpcSceneTransitionArg {
    fn from(value: SceneTransitionCsReq) -> Self {
        Self {
            section_id: value.section_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::SceneInfo> for SceneInfo {
    fn from(value: ::protocol::SceneInfo) -> Self {
        Self {
            event_id: value.event_id.into(),
            fight_scene_info: value.fight_scene_info.map(|v| v.into()),
            local_play_type: value.local_play_type.into(),
            scene_type: value.scene_type.into(),
            hall_scene_info: value.hall_scene_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<SceneInfo> for ::protocol::SceneInfo {
    fn from(value: SceneInfo) -> Self {
        Self {
            event_id: value.event_id.into(),
            fight_scene_info: value.fight_scene_info.map(|v| v.into()),
            local_play_type: value.local_play_type.into(),
            scene_type: value.scene_type.into(),
            hall_scene_info: value.hall_scene_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRedDotListRet> for GetRedDotListScRsp {
    fn from(value: ::protocol::RpcGetRedDotListRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetRedDotListScRsp> for ::protocol::RpcGetRedDotListRet {
    fn from(value: GetRedDotListScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcWorkbenchGetDataArg> for WorkbenchGetDataCsReq {
    fn from(value: ::protocol::RpcWorkbenchGetDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<WorkbenchGetDataCsReq> for ::protocol::RpcWorkbenchGetDataArg {
    fn from(value: WorkbenchGetDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RewardBuffData> for RewardBuffData {
    fn from(value: ::protocol::RewardBuffData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<RewardBuffData> for ::protocol::RewardBuffData {
    fn from(value: RewardBuffData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetResourceDataRet> for GetResourceDataScRsp {
    fn from(value: ::protocol::RpcGetResourceDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            resource_list: value.resource_list.into_iter().map(|v| v.into()).collect(),
            auto_recovery_info: value
                .auto_recovery_info
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetResourceDataScRsp> for ::protocol::RpcGetResourceDataRet {
    fn from(value: GetResourceDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            resource_list: value.resource_list.into_iter().map(|v| v.into()).collect(),
            auto_recovery_info: value
                .auto_recovery_info
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcRechargeGetItemListArg> for RechargeGetItemListCsReq {
    fn from(value: ::protocol::RpcRechargeGetItemListArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<RechargeGetItemListCsReq> for ::protocol::RpcRechargeGetItemListArg {
    fn from(value: RechargeGetItemListCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::ArchiveData> for ArchiveData {
    fn from(value: ::protocol::ArchiveData) -> Self {
        Self {
            videotaps_info: value.videotaps_info.into_iter().map(|v| v.into()).collect(),
            hollow_archive_id_list: value
                .hollow_archive_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ArchiveData> for ::protocol::ArchiveData {
    fn from(value: ArchiveData) -> Self {
        Self {
            videotaps_info: value.videotaps_info.into_iter().map(|v| v.into()).collect(),
            hollow_archive_id_list: value
                .hollow_archive_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetHollowDataArg> for GetHollowDataCsReq {
    fn from(value: ::protocol::RpcGetHollowDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetHollowDataCsReq> for ::protocol::RpcGetHollowDataArg {
    fn from(value: GetHollowDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetFriendListArg> for GetFriendListCsReq {
    fn from(value: ::protocol::RpcGetFriendListArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetFriendListCsReq> for ::protocol::RpcGetFriendListArg {
    fn from(value: GetFriendListCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcSavePosInMainCityArg> for SavePosInMainCityCsReq {
    fn from(value: ::protocol::RpcSavePosInMainCityArg) -> Self {
        Self {
            section_id: value.section_id.into(),
            position: Some(value.position.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<SavePosInMainCityCsReq> for ::protocol::RpcSavePosInMainCityArg {
    fn from(value: SavePosInMainCityCsReq) -> Self {
        Self {
            section_id: value.section_id.into(),
            position: value.position.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::PtcSyncEventInfoArg> for SyncEventInfoScNotify {
    fn from(value: ::protocol::PtcSyncEventInfoArg) -> Self {
        Self {
            owner_type: value.owner_type.into(),
            owner_id: value.owner_id.into(),
            tag: value.tag.into(),
            action_list: value.action_list.into_iter().map(|v| v.into()).collect(),
            npc_interaction: value.npc_interaction.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<SyncEventInfoScNotify> for ::protocol::PtcSyncEventInfoArg {
    fn from(value: SyncEventInfoScNotify) -> Self {
        Self {
            owner_type: value.owner_type.into(),
            owner_id: value.owner_id.into(),
            tag: value.tag.into(),
            action_list: value.action_list.into_iter().map(|v| v.into()).collect(),
            npc_interaction: value.npc_interaction.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetAbyssRewardDataRet> for GetAbyssRewardDataScRsp {
    fn from(value: ::protocol::RpcGetAbyssRewardDataRet) -> Self {
        Self {
            abyss_reward_data: Some(value.abyss_reward_data.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetAbyssRewardDataScRsp> for ::protocol::RpcGetAbyssRewardDataRet {
    fn from(value: GetAbyssRewardDataScRsp) -> Self {
        Self {
            abyss_reward_data: value
                .abyss_reward_data
                .map(|v| v.into())
                .unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::ResourceInfo> for ResourceInfo {
    fn from(value: ::protocol::ResourceInfo) -> Self {
        Self {
            id: value.id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ResourceInfo> for ::protocol::ResourceInfo {
    fn from(value: ResourceInfo) -> Self {
        Self {
            id: value.id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcModMainCityAvatarArg> for ModMainCityAvatarCsReq {
    fn from(value: ::protocol::RpcModMainCityAvatarArg) -> Self {
        Self {
            main_city_avatar_id: value.main_city_avatar_id.into(),
            player_avatar_id: value.player_avatar_id.into(),
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ModMainCityAvatarCsReq> for ::protocol::RpcModMainCityAvatarArg {
    fn from(value: ModMainCityAvatarCsReq) -> Self {
        Self {
            main_city_avatar_id: value.main_city_avatar_id.into(),
            player_avatar_id: value.player_avatar_id.into(),
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcRefreshSectionRet> for RefreshSectionScRsp {
    fn from(value: ::protocol::RpcRefreshSectionRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            refresh_status: value.refresh_status.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<RefreshSectionScRsp> for ::protocol::RpcRefreshSectionRet {
    fn from(value: RefreshSectionScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            refresh_status: value.refresh_status.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::UnlockData> for UnlockData {
    fn from(value: ::protocol::UnlockData) -> Self {
        Self {
            unlocked_list: value.unlocked_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<UnlockData> for ::protocol::UnlockData {
    fn from(value: UnlockData) -> Self {
        Self {
            unlocked_list: value.unlocked_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetWishlistDataRet> for GetWishlistDataScRsp {
    fn from(value: ::protocol::RpcGetWishlistDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetWishlistDataScRsp> for ::protocol::RpcGetWishlistDataRet {
    fn from(value: GetWishlistDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::PtcPlayerSyncArg> for PlayerSyncScNotify {
    fn from(value: ::protocol::PtcPlayerSyncArg) -> Self {
        Self {
            basic_info: value.basic_info.map(|v| v.into()),
            avatar: value.avatar.map(|v| v.into()),
            client_systems: value.client_systems.map(|v| v.into()),
            item: value.item.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<PlayerSyncScNotify> for ::protocol::PtcPlayerSyncArg {
    fn from(value: PlayerSyncScNotify) -> Self {
        Self {
            basic_info: value.basic_info.map(|v| v.into()),
            avatar: value.avatar.map(|v| v.into()),
            client_systems: value.client_systems.map(|v| v.into()),
            item: value.item.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetWeaponDataRet> for GetWeaponDataScRsp {
    fn from(value: ::protocol::RpcGetWeaponDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            weapon_list: value.weapon_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetWeaponDataScRsp> for ::protocol::RpcGetWeaponDataRet {
    fn from(value: GetWeaponDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            weapon_list: value.weapon_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcBeginTrainingCourseBattleArg>
for BeginTrainingCourseBattleCsReq {
    fn from(value: ::protocol::RpcBeginTrainingCourseBattleArg) -> Self {
        Self {
            buddy_id: value.buddy_id.into(),
            quest_id: value.quest_id.into(),
            avatars: value.avatars.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<BeginTrainingCourseBattleCsReq>
for ::protocol::RpcBeginTrainingCourseBattleArg {
    fn from(value: BeginTrainingCourseBattleCsReq) -> Self {
        Self {
            buddy_id: value.buddy_id.into(),
            quest_id: value.quest_id.into(),
            avatars: value.avatars.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetCafeDataRet> for GetCafeDataScRsp {
    fn from(value: ::protocol::RpcGetCafeDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            cafe_data: Some(value.cafe_data.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetCafeDataScRsp> for ::protocol::RpcGetCafeDataRet {
    fn from(value: GetCafeDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            cafe_data: value.cafe_data.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetCampIdleDataRet> for GetCampIdleDataScRsp {
    fn from(value: ::protocol::RpcGetCampIdleDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            camp_idle_data: Some(value.camp_idle_data.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetCampIdleDataScRsp> for ::protocol::RpcGetCampIdleDataRet {
    fn from(value: GetCampIdleDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            camp_idle_data: value.camp_idle_data.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcRunEventGraphArg> for RunEventGraphCsReq {
    fn from(value: ::protocol::RpcRunEventGraphArg) -> Self {
        Self {
            owner_id: value.owner_id.into(),
            section_id: value.section_id.into(),
            owner_type: value.owner_type.into(),
            event_graph_uid: value.event_graph_uid.into(),
            tag: value.tag.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<RunEventGraphCsReq> for ::protocol::RpcRunEventGraphArg {
    fn from(value: RunEventGraphCsReq) -> Self {
        Self {
            owner_id: value.owner_id.into(),
            section_id: value.section_id.into(),
            owner_type: value.owner_type.into(),
            event_graph_uid: value.event_graph_uid.into(),
            tag: value.tag.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetFairyDataArg> for GetFairyDataCsReq {
    fn from(value: ::protocol::RpcGetFairyDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetFairyDataCsReq> for ::protocol::RpcGetFairyDataArg {
    fn from(value: GetFairyDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcEndBattleArg> for EndBattleCsReq {
    fn from(value: ::protocol::RpcEndBattleArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<EndBattleCsReq> for ::protocol::RpcEndBattleArg {
    fn from(value: EndBattleCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::AbyssRewardData> for AbyssRewardData {
    fn from(value: ::protocol::AbyssRewardData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<AbyssRewardData> for ::protocol::AbyssRewardData {
    fn from(value: AbyssRewardData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::FightSceneInfo> for FightSceneInfo {
    fn from(value: ::protocol::FightSceneInfo) -> Self {
        Self {
            level_reward_info: Some(value.level_reward_info.into()),
            end_hollow: value.end_hollow.into(),
            level_perform_info: Some(value.level_perform_info.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<FightSceneInfo> for ::protocol::FightSceneInfo {
    fn from(value: FightSceneInfo) -> Self {
        Self {
            level_reward_info: value
                .level_reward_info
                .map(|v| v.into())
                .unwrap_or_default(),
            end_hollow: value.end_hollow.into(),
            level_perform_info: value
                .level_perform_info
                .map(|v| v.into())
                .unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::ActionInfo> for ActionInfo {
    fn from(value: ::protocol::ActionInfo) -> Self {
        Self {
            body: value.body.into_iter().map(|v| v.into()).collect(),
            action_type: value.action_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ActionInfo> for ::protocol::ActionInfo {
    fn from(value: ActionInfo) -> Self {
        Self {
            body: value.body.into_iter().map(|v| v.into()).collect(),
            action_type: value.action_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetOnlineFriendsListRet> for GetOnlineFriendsListScRsp {
    fn from(value: ::protocol::RpcGetOnlineFriendsListRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetOnlineFriendsListScRsp> for ::protocol::RpcGetOnlineFriendsListRet {
    fn from(value: GetOnlineFriendsListScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPlayerBasicInfoRet> for GetPlayerBasicInfoScRsp {
    fn from(value: ::protocol::RpcGetPlayerBasicInfoRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            basic_info: Some(value.basic_info.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetPlayerBasicInfoScRsp> for ::protocol::RpcGetPlayerBasicInfoRet {
    fn from(value: GetPlayerBasicInfoScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            basic_info: value.basic_info.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::SceneUnitProtocolInfo> for SceneUnitProtocolInfo {
    fn from(value: ::protocol::SceneUnitProtocolInfo) -> Self {
        Self {
            interacts_info: value
                .interacts_info
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            is_interactable: value.is_interactable.into(),
            npc_id: value.npc_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<SceneUnitProtocolInfo> for ::protocol::SceneUnitProtocolInfo {
    fn from(value: SceneUnitProtocolInfo) -> Self {
        Self {
            interacts_info: value
                .interacts_info
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            is_interactable: value.is_interactable.into(),
            npc_id: value.npc_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcSetLanguageArg> for SetLanguageCsReq {
    fn from(value: ::protocol::RpcSetLanguageArg) -> Self {
        Self {
            language: value.language.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<SetLanguageCsReq> for ::protocol::RpcSetLanguageArg {
    fn from(value: SetLanguageCsReq) -> Self {
        Self {
            language: value.language.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetShoppingMallInfoArg> for GetShoppingMallInfoCsReq {
    fn from(value: ::protocol::RpcGetShoppingMallInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetShoppingMallInfoCsReq> for ::protocol::RpcGetShoppingMallInfoArg {
    fn from(value: GetShoppingMallInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRoleCardDataRet> for GetRoleCardDataScRsp {
    fn from(value: ::protocol::RpcGetRoleCardDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            role_card_data: Some(value.role_card_data.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetRoleCardDataScRsp> for ::protocol::RpcGetRoleCardDataRet {
    fn from(value: GetRoleCardDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            role_card_data: value.role_card_data.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetChatEmojiListRet> for GetChatEmojiListScRsp {
    fn from(value: ::protocol::RpcGetChatEmojiListRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetChatEmojiListScRsp> for ::protocol::RpcGetChatEmojiListRet {
    fn from(value: GetChatEmojiListScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::Transform> for Transform {
    fn from(value: ::protocol::Transform) -> Self {
        Self {
            position: value.position.into_iter().map(|v| v.into()).collect(),
            rotation: value.rotation.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<Transform> for ::protocol::Transform {
    fn from(value: Transform) -> Self {
        Self {
            position: value.position.into_iter().map(|v| v.into()).collect(),
            rotation: value.rotation.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::ItemSync> for ItemSync {
    fn from(value: ::protocol::ItemSync) -> Self {
        Self {
            auto_recovery_info: value
                .auto_recovery_info
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            weapon_list: value.weapon_list.into_iter().map(|v| v.into()).collect(),
            resource_list: value.resource_list.into_iter().map(|v| v.into()).collect(),
            equip_list: value.equip_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ItemSync> for ::protocol::ItemSync {
    fn from(value: ItemSync) -> Self {
        Self {
            auto_recovery_info: value
                .auto_recovery_info
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            weapon_list: value.weapon_list.into_iter().map(|v| v.into()).collect(),
            resource_list: value.resource_list.into_iter().map(|v| v.into()).collect(),
            equip_list: value.equip_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcLeaveCurSceneArg> for LeaveCurSceneCsReq {
    fn from(value: ::protocol::RpcLeaveCurSceneArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<LeaveCurSceneCsReq> for ::protocol::RpcLeaveCurSceneArg {
    fn from(value: LeaveCurSceneCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRidusGotBooDataArg> for GetRidusGotBooDataCsReq {
    fn from(value: ::protocol::RpcGetRidusGotBooDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetRidusGotBooDataCsReq> for ::protocol::RpcGetRidusGotBooDataArg {
    fn from(value: GetRidusGotBooDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetCollectMapArg> for GetCollectMapCsReq {
    fn from(value: ::protocol::RpcGetCollectMapArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetCollectMapCsReq> for ::protocol::RpcGetCollectMapArg {
    fn from(value: GetCollectMapCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcPlayerTransactionRet> for PlayerTransactionScRsp {
    fn from(value: ::protocol::RpcPlayerTransactionRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<PlayerTransactionScRsp> for ::protocol::RpcPlayerTransactionRet {
    fn from(value: PlayerTransactionScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::PtcEnterSceneArg> for EnterSceneScNotify {
    fn from(value: ::protocol::PtcEnterSceneArg) -> Self {
        Self {
            scene_info: Some(value.scene_info.into()),
            dungeon_info: value.dungeon_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<EnterSceneScNotify> for ::protocol::PtcEnterSceneArg {
    fn from(value: EnterSceneScNotify) -> Self {
        Self {
            scene_info: value.scene_info.map(|v| v.into()).unwrap_or_default(),
            dungeon_info: value.dungeon_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcWeaponDressArg> for WeaponDressCsReq {
    fn from(value: ::protocol::RpcWeaponDressArg) -> Self {
        Self {
            weapon_uid: value.weapon_uid.into(),
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<WeaponDressCsReq> for ::protocol::RpcWeaponDressArg {
    fn from(value: WeaponDressCsReq) -> Self {
        Self {
            weapon_uid: value.weapon_uid.into(),
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetMonthCardRewardListArg> for GetMonthCardRewardListCsReq {
    fn from(value: ::protocol::RpcGetMonthCardRewardListArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetMonthCardRewardListCsReq> for ::protocol::RpcGetMonthCardRewardListArg {
    fn from(value: GetMonthCardRewardListCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcPlayerLoginArg> for PlayerLoginCsReq {
    fn from(value: ::protocol::RpcPlayerLoginArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<PlayerLoginCsReq> for ::protocol::RpcPlayerLoginArg {
    fn from(value: PlayerLoginCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetHadalZoneDataArg> for GetHadalZoneDataCsReq {
    fn from(value: ::protocol::RpcGetHadalZoneDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetHadalZoneDataCsReq> for ::protocol::RpcGetHadalZoneDataArg {
    fn from(value: GetHadalZoneDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcSavePlayerSystemSettingArg> for SavePlayerSystemSettingCsReq {
    fn from(value: ::protocol::RpcSavePlayerSystemSettingArg) -> Self {
        Self {
            r#type: value.r#type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<SavePlayerSystemSettingCsReq> for ::protocol::RpcSavePlayerSystemSettingArg {
    fn from(value: SavePlayerSystemSettingCsReq) -> Self {
        Self {
            r#type: value.r#type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetVhsStoreDataRet> for GetVhsStoreDataScRsp {
    fn from(value: ::protocol::RpcGetVhsStoreDataRet) -> Self {
        Self {
            data: Some(value.data.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetVhsStoreDataScRsp> for ::protocol::RpcGetVhsStoreDataRet {
    fn from(value: GetVhsStoreDataScRsp) -> Self {
        Self {
            data: value.data.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetWebActivityDataRet> for GetWebActivityDataScRsp {
    fn from(value: ::protocol::RpcGetWebActivityDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetWebActivityDataScRsp> for ::protocol::RpcGetWebActivityDataRet {
    fn from(value: GetWebActivityDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::PostGirlSyncData> for PostGirlSyncData {
    fn from(value: ::protocol::PostGirlSyncData) -> Self {
        Self {
            show_random_selected: value.show_random_selected.into(),
            selected_post_girl_id_list: value
                .selected_post_girl_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            add_post_girl_item_list: value
                .add_post_girl_item_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<PostGirlSyncData> for ::protocol::PostGirlSyncData {
    fn from(value: PostGirlSyncData) -> Self {
        Self {
            show_random_selected: value.show_random_selected.into(),
            selected_post_girl_id_list: value
                .selected_post_girl_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            add_post_girl_item_list: value
                .add_post_girl_item_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::FashionStoreData> for FashionStoreData {
    fn from(value: ::protocol::FashionStoreData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<FashionStoreData> for ::protocol::FashionStoreData {
    fn from(value: FashionStoreData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetAvatarDataArg> for GetAvatarDataCsReq {
    fn from(value: ::protocol::RpcGetAvatarDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetAvatarDataCsReq> for ::protocol::RpcGetAvatarDataArg {
    fn from(value: GetAvatarDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetTrashbinHermitDataArg> for GetTrashbinHermitDataCsReq {
    fn from(value: ::protocol::RpcGetTrashbinHermitDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetTrashbinHermitDataCsReq> for ::protocol::RpcGetTrashbinHermitDataArg {
    fn from(value: GetTrashbinHermitDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[macro_export]
macro_rules! decode_and_forward_proto {
    (
        $cmd_id:expr, $buf:expr, $session:expr, $point:expr, $addr:expr,
        $middlewares:expr, $timeout:expr
    ) => {
        match $cmd_id { PlayerOperationCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::PlayerOperationCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcPlayerOperationArg::from(packet.body); let rpc_ret :
        ::protocol::RpcPlayerOperationRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetNewsStandDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetNewsStandDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetNewsStandDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetNewsStandDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetNewsStandDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, ModTimeCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::ModTimeCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcModTimeArg::from(packet.body); let rpc_ret :
        ::protocol::RpcModTimeRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetRamenDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetRamenDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetRamenDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetRamenDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetRamenDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetExplorationDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetExplorationDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetExplorationDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetExplorationDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetExplorationDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, RefreshSectionCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::RefreshSectionCsReq > ::decode($buf) ?; let rpc_arg
        = ::protocol::RpcRefreshSectionArg::from(packet.body); let rpc_ret :
        ::protocol::RpcRefreshSectionRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::RefreshSectionScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetDailyChallengeDataCsReq::CMD_ID => { let
        packet = NetPacket:: < ::evelyn_proto::GetDailyChallengeDataCsReq >
        ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetDailyChallengeDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetDailyChallengeDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetDailyChallengeDataScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, GetEmbattlesDataCsReq::CMD_ID =>
        { let packet = NetPacket:: < ::evelyn_proto::GetEmbattlesDataCsReq >
        ::decode($buf) ?; let rpc_arg = ::protocol::RpcGetEmbattlesDataArg::from(packet
        .body); let rpc_ret : ::protocol::RpcGetEmbattlesDataRet = $point
        .call_rpc($addr, rpc_arg, $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetEmbattlesDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetCampIdleDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetCampIdleDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetCampIdleDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetCampIdleDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetCampIdleDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, KeepAliveNotify::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::KeepAliveNotify > ::decode($buf) ?; let rpc_arg =
        ::protocol::PtcKeepAliveArg::from(packet.body); $point .send_rpc($addr, 0,
        rpc_arg, 0, $middlewares). await; } GetOnlineFriendsListCsReq::CMD_ID => { let
        packet = NetPacket:: < ::evelyn_proto::GetOnlineFriendsListCsReq > ::decode($buf)
        ?; let rpc_arg = ::protocol::RpcGetOnlineFriendsListArg::from(packet.body); let
        rpc_ret : ::protocol::RpcGetOnlineFriendsListRet = $point .call_rpc($addr,
        rpc_arg, $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetOnlineFriendsListScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, EnterSectionCsReq::CMD_ID => {
        let packet = NetPacket:: < ::evelyn_proto::EnterSectionCsReq > ::decode($buf) ?;
        let rpc_arg = ::protocol::RpcEnterSectionArg::from(packet.body); let rpc_ret :
        ::protocol::RpcEnterSectionRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetCafeDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetCafeDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetCafeDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetCafeDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetCafeDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetPlayerMailsCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetPlayerMailsCsReq > ::decode($buf) ?; let rpc_arg
        = ::protocol::RpcGetPlayerMailsArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetPlayerMailsRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetRoleCardDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetRoleCardDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetRoleCardDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetRoleCardDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetRoleCardDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetGachaDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetGachaDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetGachaDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetGachaDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetActivityDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetActivityDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetActivityDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetActivityDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetActivityDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetPlayerNetworkDataCsReq::CMD_ID => { let packet
        = NetPacket:: < ::evelyn_proto::GetPlayerNetworkDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetPlayerNetworkDataArg::from(packet.body); let rpc_ret
        : ::protocol::RpcGetPlayerNetworkDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetWebActivityDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetWebActivityDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetWebActivityDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetWebActivityDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetWebActivityDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetAbyssRewardDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetAbyssRewardDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetAbyssRewardDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetAbyssRewardDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetAbyssRewardDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, ReportEmbattleInfoCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::ReportEmbattleInfoCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcReportEmbattleInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcReportEmbattleInfoRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetPhotoWallDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetPhotoWallDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetPhotoWallDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetPhotoWallDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, PostEnterWorldCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::PostEnterWorldCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcPostEnterWorldArg::from(packet.body); let rpc_ret :
        ::protocol::RpcPostEnterWorldRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        WeaponUnDressCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::WeaponUnDressCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcWeaponUnDressArg::from(packet.body); let rpc_ret :
        ::protocol::RpcWeaponUnDressRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetMiniscapeEntrustDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetMiniscapeEntrustDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetMiniscapeEntrustDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetMiniscapeEntrustDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetMiniscapeEntrustDataScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, GetAuthkeyCsReq::CMD_ID => { let
        packet = NetPacket:: < ::evelyn_proto::GetAuthkeyCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetAuthkeyArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetAuthkeyRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetAuthkeyScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetQuestDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetQuestDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetQuestDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetQuestDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetQuestDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetTipsInfoCsReq::CMD_ID => { let packet = NetPacket::
        < ::evelyn_proto::GetTipsInfoCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetTipsInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetTipsInfoRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetTipsInfoScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetWishlistDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetWishlistDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetWishlistDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetWishlistDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetWishlistDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetClientSystemsDataCsReq::CMD_ID => { let packet
        = NetPacket:: < ::evelyn_proto::GetClientSystemsDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetClientSystemsDataArg::from(packet.body); let rpc_ret
        : ::protocol::RpcGetClientSystemsDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetClientSystemsDataScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, GetBabelTowerDataCsReq::CMD_ID =>
        { let packet = NetPacket:: < ::evelyn_proto::GetBabelTowerDataCsReq >
        ::decode($buf) ?; let rpc_arg = ::protocol::RpcGetBabelTowerDataArg::from(packet
        .body); let rpc_ret : ::protocol::RpcGetBabelTowerDataRet = $point
        .call_rpc($addr, rpc_arg, $middlewares, $timeout). await ?; $session
        .send_null_rsp(packet.head.packet_id); }, GetVhsStoreDataCsReq::CMD_ID => { let
        packet = NetPacket:: < ::evelyn_proto::GetVhsStoreDataCsReq > ::decode($buf) ?;
        let rpc_arg = ::protocol::RpcGetVhsStoreDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetVhsStoreDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetVhsStoreDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetServerTimestampCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetServerTimestampCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetServerTimestampArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetServerTimestampRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, SelectPostGirlCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::SelectPostGirlCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcSelectPostGirlArg::from(packet.body); let rpc_ret :
        ::protocol::RpcSelectPostGirlRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetRedDotListCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetRedDotListCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetRedDotListArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetRedDotListRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetRedDotListScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetRewardBuffDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetRewardBuffDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetRewardBuffDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetRewardBuffDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetRewardBuffDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetBuddyDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetBuddyDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetBuddyDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetBuddyDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetMainCityRevivalDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetMainCityRevivalDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetMainCityRevivalDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetMainCityRevivalDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetMainCityRevivalDataScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, GetArchiveDataCsReq::CMD_ID => {
        let packet = NetPacket:: < ::evelyn_proto::GetArchiveDataCsReq > ::decode($buf)
        ?; let rpc_arg = ::protocol::RpcGetArchiveDataArg::from(packet.body); let rpc_ret
        : ::protocol::RpcGetArchiveDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetArchiveDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetArcadeDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetArcadeDataCsReq > ::decode($buf) ?; let rpc_arg
        = ::protocol::RpcGetArcadeDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetArcadeDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetArcadeDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, VideoGetInfoCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::VideoGetInfoCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcVideoGetInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcVideoGetInfoRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::VideoGetInfoScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, CheckYorozuyaInfoRefreshCsReq::CMD_ID => { let packet
        = NetPacket:: < ::evelyn_proto::CheckYorozuyaInfoRefreshCsReq > ::decode($buf) ?;
        let rpc_arg = ::protocol::RpcCheckYorozuyaInfoRefreshArg::from(packet.body); let
        rpc_ret : ::protocol::RpcCheckYorozuyaInfoRefreshRet = $point .call_rpc($addr,
        rpc_arg, $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head
        .packet_id); }, InteractWithUnitCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::InteractWithUnitCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcInteractWithUnitArg::from(packet.body); let rpc_ret :
        ::protocol::RpcInteractWithUnitRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetFishingContestDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetFishingContestDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetFishingContestDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetFishingContestDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetFishingContestDataScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, GetEquipDataCsReq::CMD_ID => {
        let packet = NetPacket:: < ::evelyn_proto::GetEquipDataCsReq > ::decode($buf) ?;
        let rpc_arg = ::protocol::RpcGetEquipDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetEquipDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetEquipDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetPlayerBasicInfoCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetPlayerBasicInfoCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetPlayerBasicInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetPlayerBasicInfoRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetPlayerBasicInfoScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetResourceDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetResourceDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetResourceDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetResourceDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetResourceDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, EnterWorldCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::EnterWorldCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcEnterWorldArg::from(packet.body); let rpc_ret :
        ::protocol::RpcEnterWorldRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::EnterWorldScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, BattleReportCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::BattleReportCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcBattleReportArg::from(packet.body); let rpc_ret :
        ::protocol::RpcBattleReportRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetCharacterQuestListCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetCharacterQuestListCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetCharacterQuestListArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetCharacterQuestListRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetCharacterQuestListScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, EnterSectionCompleteCsReq::CMD_ID
        => { let packet = NetPacket:: < ::evelyn_proto::EnterSectionCompleteCsReq >
        ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcEnterSectionCompleteArg::from(packet.body); let rpc_ret :
        ::protocol::RpcEnterSectionCompleteRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::EnterSectionCompleteScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, GetChatEmojiListCsReq::CMD_ID =>
        { let packet = NetPacket:: < ::evelyn_proto::GetChatEmojiListCsReq >
        ::decode($buf) ?; let rpc_arg = ::protocol::RpcGetChatEmojiListArg::from(packet
        .body); let rpc_ret : ::protocol::RpcGetChatEmojiListRet = $point
        .call_rpc($addr, rpc_arg, $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetChatEmojiListScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, AbyssArpeggioGetDataCsReq::CMD_ID => { let packet
        = NetPacket:: < ::evelyn_proto::AbyssArpeggioGetDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcAbyssArpeggioGetDataArg::from(packet.body); let rpc_ret
        : ::protocol::RpcAbyssArpeggioGetDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetPrivateMessageDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetPrivateMessageDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetPrivateMessageDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetPrivateMessageDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetPrivateMessageDataScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, PlayerTransactionCsReq::CMD_ID =>
        { let packet = NetPacket:: < ::evelyn_proto::PlayerTransactionCsReq >
        ::decode($buf) ?; let rpc_arg = ::protocol::RpcPlayerTransactionArg::from(packet
        .body); let rpc_ret : ::protocol::RpcPlayerTransactionRet = $point
        .call_rpc($addr, rpc_arg, $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::PlayerTransactionScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetAvatarRecommendEquipCsReq::CMD_ID => { let
        packet = NetPacket:: < ::evelyn_proto::GetAvatarRecommendEquipCsReq >
        ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetAvatarRecommendEquipArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetAvatarRecommendEquipRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, InteractWithClientEntityCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::InteractWithClientEntityCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcInteractWithClientEntityArg::from(packet.body); let rpc_ret :
        ::protocol::RpcInteractWithClientEntityRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, ReportUiLayoutPlatformCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::ReportUiLayoutPlatformCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcReportUiLayoutPlatformArg::from(packet.body); let rpc_ret :
        ::protocol::RpcReportUiLayoutPlatformRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, AbyssGetDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::AbyssGetDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcAbyssGetDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcAbyssGetDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetJourneyDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetJourneyDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetJourneyDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetJourneyDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetJourneyDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetBattlePassDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetBattlePassDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetBattlePassDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetBattlePassDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetBattlePassDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetWeaponDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetWeaponDataCsReq > ::decode($buf) ?; let rpc_arg
        = ::protocol::RpcGetWeaponDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetWeaponDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetWeaponDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetDisplayCaseDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetDisplayCaseDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetDisplayCaseDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetDisplayCaseDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetFashionStoreDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetFashionStoreDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetFashionStoreDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetFashionStoreDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetFashionStoreDataScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, SceneTransitionCsReq::CMD_ID => {
        let packet = NetPacket:: < ::evelyn_proto::SceneTransitionCsReq > ::decode($buf)
        ?; let rpc_arg = ::protocol::RpcSceneTransitionArg::from(packet.body); let
        rpc_ret : ::protocol::RpcSceneTransitionRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, WorkbenchGetDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::WorkbenchGetDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcWorkbenchGetDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcWorkbenchGetDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::WorkbenchGetDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, RechargeGetItemListCsReq::CMD_ID => { let packet
        = NetPacket:: < ::evelyn_proto::RechargeGetItemListCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcRechargeGetItemListArg::from(packet.body); let rpc_ret :
        ::protocol::RpcRechargeGetItemListRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::RechargeGetItemListScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, GetHollowDataCsReq::CMD_ID => {
        let packet = NetPacket:: < ::evelyn_proto::GetHollowDataCsReq > ::decode($buf) ?;
        let rpc_arg = ::protocol::RpcGetHollowDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetHollowDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetHollowDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetFriendListCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetFriendListCsReq > ::decode($buf) ?; let rpc_arg
        = ::protocol::RpcGetFriendListArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetFriendListRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetFriendListScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, SavePosInMainCityCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::SavePosInMainCityCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcSavePosInMainCityArg::from(packet.body); let rpc_ret :
        ::protocol::RpcSavePosInMainCityRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, ModMainCityAvatarCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::ModMainCityAvatarCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcModMainCityAvatarArg::from(packet.body); let rpc_ret :
        ::protocol::RpcModMainCityAvatarRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, BeginTrainingCourseBattleCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::BeginTrainingCourseBattleCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcBeginTrainingCourseBattleArg::from(packet.body); let rpc_ret :
        ::protocol::RpcBeginTrainingCourseBattleRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, RunEventGraphCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::RunEventGraphCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcRunEventGraphArg::from(packet.body); let rpc_ret :
        ::protocol::RpcRunEventGraphRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::RunEventGraphScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetFairyDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetFairyDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetFairyDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetFairyDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetFairyDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, EndBattleCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::EndBattleCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcEndBattleArg::from(packet.body); let rpc_ret :
        ::protocol::RpcEndBattleRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::EndBattleScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, SetLanguageCsReq::CMD_ID => { let packet = NetPacket::
        < ::evelyn_proto::SetLanguageCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcSetLanguageArg::from(packet.body); let rpc_ret :
        ::protocol::RpcSetLanguageRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetShoppingMallInfoCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetShoppingMallInfoCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetShoppingMallInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetShoppingMallInfoRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetShoppingMallInfoScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, LeaveCurSceneCsReq::CMD_ID => {
        let packet = NetPacket:: < ::evelyn_proto::LeaveCurSceneCsReq > ::decode($buf) ?;
        let rpc_arg = ::protocol::RpcLeaveCurSceneArg::from(packet.body); let rpc_ret :
        ::protocol::RpcLeaveCurSceneRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetRidusGotBooDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetRidusGotBooDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetRidusGotBooDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetRidusGotBooDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetRidusGotBooDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetCollectMapCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetCollectMapCsReq > ::decode($buf) ?; let rpc_arg
        = ::protocol::RpcGetCollectMapArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetCollectMapRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetCollectMapScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, WeaponDressCsReq::CMD_ID => { let packet = NetPacket::
        < ::evelyn_proto::WeaponDressCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcWeaponDressArg::from(packet.body); let rpc_ret :
        ::protocol::RpcWeaponDressRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetMonthCardRewardListCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetMonthCardRewardListCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetMonthCardRewardListArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetMonthCardRewardListRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, PlayerLoginCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::PlayerLoginCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcPlayerLoginArg::from(packet.body); let rpc_ret :
        ::protocol::RpcPlayerLoginRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::PlayerLoginScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetHadalZoneDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetHadalZoneDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetHadalZoneDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetHadalZoneDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, SavePlayerSystemSettingCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::SavePlayerSystemSettingCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcSavePlayerSystemSettingArg::from(packet.body); let rpc_ret :
        ::protocol::RpcSavePlayerSystemSettingRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetAvatarDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::evelyn_proto::GetAvatarDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetAvatarDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetAvatarDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetAvatarDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetTrashbinHermitDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::evelyn_proto::GetTrashbinHermitDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetTrashbinHermitDataArg::from(packet.body); let rpc_ret
        : ::protocol::RpcGetTrashbinHermitDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::evelyn_proto::GetTrashbinHermitDataScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, _ =>
        ::tracing::warn!("unknown cmd_id: {}", $cmd_id), }
    };
}
#[macro_export]
macro_rules! impl_qwer_to_proto_match {
    ($process_proto_message:ident) => {
        match qwer.get_protocol_id() { ::protocol::RpcPlayerOperationArg::PROTOCOL_ID =>
        $process_proto_message (PlayerOperationCsReq::from(qwer)),
        ::protocol::RpcGetNewsStandDataArg::PROTOCOL_ID => $process_proto_message
        (GetNewsStandDataCsReq::from(qwer)), ::protocol::RpcModTimeArg::PROTOCOL_ID =>
        $process_proto_message (ModTimeCsReq::from(qwer)),
        ::protocol::RpcGetRamenDataArg::PROTOCOL_ID => $process_proto_message
        (GetRamenDataCsReq::from(qwer)),
        ::protocol::RpcGetExplorationDataArg::PROTOCOL_ID => $process_proto_message
        (GetExplorationDataCsReq::from(qwer)),
        ::protocol::RpcRefreshSectionArg::PROTOCOL_ID => $process_proto_message
        (RefreshSectionCsReq::from(qwer)),
        ::protocol::RpcGetDailyChallengeDataArg::PROTOCOL_ID => $process_proto_message
        (GetDailyChallengeDataCsReq::from(qwer)),
        ::protocol::RpcGetEmbattlesDataArg::PROTOCOL_ID => $process_proto_message
        (GetEmbattlesDataCsReq::from(qwer)),
        ::protocol::RpcGetCampIdleDataArg::PROTOCOL_ID => $process_proto_message
        (GetCampIdleDataCsReq::from(qwer)), ::protocol::PtcKeepAliveArg::PROTOCOL_ID =>
        $process_proto_message (KeepAliveNotify::from(qwer)),
        ::protocol::RpcGetOnlineFriendsListArg::PROTOCOL_ID => $process_proto_message
        (GetOnlineFriendsListCsReq::from(qwer)),
        ::protocol::RpcEnterSectionArg::PROTOCOL_ID => $process_proto_message
        (EnterSectionCsReq::from(qwer)), ::protocol::RpcGetCafeDataArg::PROTOCOL_ID =>
        $process_proto_message (GetCafeDataCsReq::from(qwer)),
        ::protocol::RpcGetPlayerMailsArg::PROTOCOL_ID => $process_proto_message
        (GetPlayerMailsCsReq::from(qwer)), ::protocol::RpcGetRoleCardDataArg::PROTOCOL_ID
        => $process_proto_message (GetRoleCardDataCsReq::from(qwer)),
        ::protocol::RpcGetGachaDataArg::PROTOCOL_ID => $process_proto_message
        (GetGachaDataCsReq::from(qwer)), ::protocol::RpcGetActivityDataArg::PROTOCOL_ID
        => $process_proto_message (GetActivityDataCsReq::from(qwer)),
        ::protocol::RpcGetPlayerNetworkDataArg::PROTOCOL_ID => $process_proto_message
        (GetPlayerNetworkDataCsReq::from(qwer)),
        ::protocol::RpcGetWebActivityDataArg::PROTOCOL_ID => $process_proto_message
        (GetWebActivityDataCsReq::from(qwer)),
        ::protocol::RpcGetAbyssRewardDataArg::PROTOCOL_ID => $process_proto_message
        (GetAbyssRewardDataCsReq::from(qwer)),
        ::protocol::RpcReportEmbattleInfoArg::PROTOCOL_ID => $process_proto_message
        (ReportEmbattleInfoCsReq::from(qwer)),
        ::protocol::RpcGetPhotoWallDataArg::PROTOCOL_ID => $process_proto_message
        (GetPhotoWallDataCsReq::from(qwer)),
        ::protocol::RpcPostEnterWorldArg::PROTOCOL_ID => $process_proto_message
        (PostEnterWorldCsReq::from(qwer)), ::protocol::RpcWeaponUnDressArg::PROTOCOL_ID
        => $process_proto_message (WeaponUnDressCsReq::from(qwer)),
        ::protocol::RpcGetMiniscapeEntrustDataArg::PROTOCOL_ID => $process_proto_message
        (GetMiniscapeEntrustDataCsReq::from(qwer)),
        ::protocol::RpcGetAuthkeyArg::PROTOCOL_ID => $process_proto_message
        (GetAuthkeyCsReq::from(qwer)), ::protocol::RpcGetQuestDataArg::PROTOCOL_ID =>
        $process_proto_message (GetQuestDataCsReq::from(qwer)),
        ::protocol::RpcGetTipsInfoArg::PROTOCOL_ID => $process_proto_message
        (GetTipsInfoCsReq::from(qwer)), ::protocol::RpcGetWishlistDataArg::PROTOCOL_ID =>
        $process_proto_message (GetWishlistDataCsReq::from(qwer)),
        ::protocol::RpcGetClientSystemsDataArg::PROTOCOL_ID => $process_proto_message
        (GetClientSystemsDataCsReq::from(qwer)),
        ::protocol::RpcGetBabelTowerDataArg::PROTOCOL_ID => $process_proto_message
        (GetBabelTowerDataCsReq::from(qwer)),
        ::protocol::RpcGetVhsStoreDataArg::PROTOCOL_ID => $process_proto_message
        (GetVhsStoreDataCsReq::from(qwer)),
        ::protocol::RpcGetServerTimestampArg::PROTOCOL_ID => $process_proto_message
        (GetServerTimestampCsReq::from(qwer)),
        ::protocol::RpcSelectPostGirlArg::PROTOCOL_ID => $process_proto_message
        (SelectPostGirlCsReq::from(qwer)), ::protocol::RpcGetRedDotListArg::PROTOCOL_ID
        => $process_proto_message (GetRedDotListCsReq::from(qwer)),
        ::protocol::RpcGetRewardBuffDataArg::PROTOCOL_ID => $process_proto_message
        (GetRewardBuffDataCsReq::from(qwer)), ::protocol::PtcHallRefreshArg::PROTOCOL_ID
        => $process_proto_message (HallRefreshScNotify::from(qwer)),
        ::protocol::RpcGetBuddyDataArg::PROTOCOL_ID => $process_proto_message
        (GetBuddyDataCsReq::from(qwer)), ::protocol::PtcUpdateEventGraphArg::PROTOCOL_ID
        => $process_proto_message (UpdateEventGraphScNotify::from(qwer)),
        ::protocol::RpcGetMainCityRevivalDataArg::PROTOCOL_ID => $process_proto_message
        (GetMainCityRevivalDataCsReq::from(qwer)),
        ::protocol::RpcGetArchiveDataArg::PROTOCOL_ID => $process_proto_message
        (GetArchiveDataCsReq::from(qwer)), ::protocol::RpcGetArcadeDataArg::PROTOCOL_ID
        => $process_proto_message (GetArcadeDataCsReq::from(qwer)),
        ::protocol::RpcVideoGetInfoArg::PROTOCOL_ID => $process_proto_message
        (VideoGetInfoCsReq::from(qwer)),
        ::protocol::RpcCheckYorozuyaInfoRefreshArg::PROTOCOL_ID => $process_proto_message
        (CheckYorozuyaInfoRefreshCsReq::from(qwer)),
        ::protocol::RpcInteractWithUnitArg::PROTOCOL_ID => $process_proto_message
        (InteractWithUnitCsReq::from(qwer)),
        ::protocol::RpcGetFishingContestDataArg::PROTOCOL_ID => $process_proto_message
        (GetFishingContestDataCsReq::from(qwer)),
        ::protocol::RpcGetEquipDataArg::PROTOCOL_ID => $process_proto_message
        (GetEquipDataCsReq::from(qwer)),
        ::protocol::RpcGetPlayerBasicInfoArg::PROTOCOL_ID => $process_proto_message
        (GetPlayerBasicInfoCsReq::from(qwer)),
        ::protocol::RpcGetResourceDataArg::PROTOCOL_ID => $process_proto_message
        (GetResourceDataCsReq::from(qwer)), ::protocol::RpcEnterWorldArg::PROTOCOL_ID =>
        $process_proto_message (EnterWorldCsReq::from(qwer)),
        ::protocol::RpcBattleReportArg::PROTOCOL_ID => $process_proto_message
        (BattleReportCsReq::from(qwer)),
        ::protocol::RpcGetCharacterQuestListArg::PROTOCOL_ID => $process_proto_message
        (GetCharacterQuestListCsReq::from(qwer)),
        ::protocol::RpcEnterSectionCompleteArg::PROTOCOL_ID => $process_proto_message
        (EnterSectionCompleteCsReq::from(qwer)),
        ::protocol::RpcGetChatEmojiListArg::PROTOCOL_ID => $process_proto_message
        (GetChatEmojiListCsReq::from(qwer)),
        ::protocol::RpcAbyssArpeggioGetDataArg::PROTOCOL_ID => $process_proto_message
        (AbyssArpeggioGetDataCsReq::from(qwer)),
        ::protocol::RpcGetPrivateMessageDataArg::PROTOCOL_ID => $process_proto_message
        (GetPrivateMessageDataCsReq::from(qwer)),
        ::protocol::RpcPlayerTransactionArg::PROTOCOL_ID => $process_proto_message
        (PlayerTransactionCsReq::from(qwer)),
        ::protocol::RpcGetAvatarRecommendEquipArg::PROTOCOL_ID => $process_proto_message
        (GetAvatarRecommendEquipCsReq::from(qwer)),
        ::protocol::RpcInteractWithClientEntityArg::PROTOCOL_ID => $process_proto_message
        (InteractWithClientEntityCsReq::from(qwer)),
        ::protocol::RpcReportUiLayoutPlatformArg::PROTOCOL_ID => $process_proto_message
        (ReportUiLayoutPlatformCsReq::from(qwer)),
        ::protocol::RpcAbyssGetDataArg::PROTOCOL_ID => $process_proto_message
        (AbyssGetDataCsReq::from(qwer)), ::protocol::RpcGetJourneyDataArg::PROTOCOL_ID =>
        $process_proto_message (GetJourneyDataCsReq::from(qwer)),
        ::protocol::RpcGetBattlePassDataArg::PROTOCOL_ID => $process_proto_message
        (GetBattlePassDataCsReq::from(qwer)),
        ::protocol::RpcGetWeaponDataArg::PROTOCOL_ID => $process_proto_message
        (GetWeaponDataCsReq::from(qwer)),
        ::protocol::RpcGetDisplayCaseDataArg::PROTOCOL_ID => $process_proto_message
        (GetDisplayCaseDataCsReq::from(qwer)),
        ::protocol::RpcGetFashionStoreDataArg::PROTOCOL_ID => $process_proto_message
        (GetFashionStoreDataCsReq::from(qwer)),
        ::protocol::RpcSceneTransitionArg::PROTOCOL_ID => $process_proto_message
        (SceneTransitionCsReq::from(qwer)),
        ::protocol::RpcWorkbenchGetDataArg::PROTOCOL_ID => $process_proto_message
        (WorkbenchGetDataCsReq::from(qwer)),
        ::protocol::RpcRechargeGetItemListArg::PROTOCOL_ID => $process_proto_message
        (RechargeGetItemListCsReq::from(qwer)),
        ::protocol::RpcGetHollowDataArg::PROTOCOL_ID => $process_proto_message
        (GetHollowDataCsReq::from(qwer)), ::protocol::RpcGetFriendListArg::PROTOCOL_ID =>
        $process_proto_message (GetFriendListCsReq::from(qwer)),
        ::protocol::RpcSavePosInMainCityArg::PROTOCOL_ID => $process_proto_message
        (SavePosInMainCityCsReq::from(qwer)),
        ::protocol::PtcSyncEventInfoArg::PROTOCOL_ID => $process_proto_message
        (SyncEventInfoScNotify::from(qwer)),
        ::protocol::RpcModMainCityAvatarArg::PROTOCOL_ID => $process_proto_message
        (ModMainCityAvatarCsReq::from(qwer)), ::protocol::PtcPlayerSyncArg::PROTOCOL_ID
        => $process_proto_message (PlayerSyncScNotify::from(qwer)),
        ::protocol::RpcBeginTrainingCourseBattleArg::PROTOCOL_ID =>
        $process_proto_message (BeginTrainingCourseBattleCsReq::from(qwer)),
        ::protocol::RpcRunEventGraphArg::PROTOCOL_ID => $process_proto_message
        (RunEventGraphCsReq::from(qwer)), ::protocol::RpcGetFairyDataArg::PROTOCOL_ID =>
        $process_proto_message (GetFairyDataCsReq::from(qwer)),
        ::protocol::RpcEndBattleArg::PROTOCOL_ID => $process_proto_message
        (EndBattleCsReq::from(qwer)), ::protocol::RpcSetLanguageArg::PROTOCOL_ID =>
        $process_proto_message (SetLanguageCsReq::from(qwer)),
        ::protocol::RpcGetShoppingMallInfoArg::PROTOCOL_ID => $process_proto_message
        (GetShoppingMallInfoCsReq::from(qwer)),
        ::protocol::RpcLeaveCurSceneArg::PROTOCOL_ID => $process_proto_message
        (LeaveCurSceneCsReq::from(qwer)),
        ::protocol::RpcGetRidusGotBooDataArg::PROTOCOL_ID => $process_proto_message
        (GetRidusGotBooDataCsReq::from(qwer)),
        ::protocol::RpcGetCollectMapArg::PROTOCOL_ID => $process_proto_message
        (GetCollectMapCsReq::from(qwer)), ::protocol::PtcEnterSceneArg::PROTOCOL_ID =>
        $process_proto_message (EnterSceneScNotify::from(qwer)),
        ::protocol::RpcWeaponDressArg::PROTOCOL_ID => $process_proto_message
        (WeaponDressCsReq::from(qwer)),
        ::protocol::RpcGetMonthCardRewardListArg::PROTOCOL_ID => $process_proto_message
        (GetMonthCardRewardListCsReq::from(qwer)),
        ::protocol::RpcPlayerLoginArg::PROTOCOL_ID => $process_proto_message
        (PlayerLoginCsReq::from(qwer)), ::protocol::RpcGetHadalZoneDataArg::PROTOCOL_ID
        => $process_proto_message (GetHadalZoneDataCsReq::from(qwer)),
        ::protocol::RpcSavePlayerSystemSettingArg::PROTOCOL_ID => $process_proto_message
        (SavePlayerSystemSettingCsReq::from(qwer)),
        ::protocol::RpcGetAvatarDataArg::PROTOCOL_ID => $process_proto_message
        (GetAvatarDataCsReq::from(qwer)),
        ::protocol::RpcGetTrashbinHermitDataArg::PROTOCOL_ID => $process_proto_message
        (GetTrashbinHermitDataCsReq::from(qwer)), _ => (), }
    };
}
#[macro_export]
macro_rules! register_ptc_handlers {
    ($point:ident, $conv:ident, $tx:ident) => {
        $point .register_rpc_recv(::protocol::PtcHallRefreshArg::PROTOCOL_ID, move | ctx
        | async move { let _ = $tx .get().unwrap().send(Input::Notify($conv, ctx)).
        await; }); $point
        .register_rpc_recv(::protocol::PtcUpdateEventGraphArg::PROTOCOL_ID, move | ctx |
        async move { let _ = $tx .get().unwrap().send(Input::Notify($conv, ctx)). await;
        }); $point .register_rpc_recv(::protocol::PtcSyncEventInfoArg::PROTOCOL_ID, move
        | ctx | async move { let _ = $tx .get().unwrap().send(Input::Notify($conv, ctx)).
        await; }); $point .register_rpc_recv(::protocol::PtcPlayerSyncArg::PROTOCOL_ID,
        move | ctx | async move { let _ = $tx .get().unwrap().send(Input::Notify($conv,
        ctx)). await; }); $point
        .register_rpc_recv(::protocol::PtcEnterSceneArg::PROTOCOL_ID, move | ctx | async
        move { let _ = $tx .get().unwrap().send(Input::Notify($conv, ctx)). await; });
    };
}
#[macro_export]
macro_rules! forward_as_notify {
    ($session:ident, $ctx:ident) => {
        match $ctx .protocol_id { ::protocol::PtcHallRefreshArg::PROTOCOL_ID => {
        $session .notify(::evelyn_proto::HallRefreshScNotify::from($ctx .get_arg:: <
        ::protocol::PtcHallRefreshArg > ().unwrap(),)); },
        ::protocol::PtcUpdateEventGraphArg::PROTOCOL_ID => { $session
        .notify(::evelyn_proto::UpdateEventGraphScNotify::from($ctx .get_arg:: <
        ::protocol::PtcUpdateEventGraphArg > ().unwrap(),)); },
        ::protocol::PtcSyncEventInfoArg::PROTOCOL_ID => { $session
        .notify(::evelyn_proto::SyncEventInfoScNotify::from($ctx .get_arg:: <
        ::protocol::PtcSyncEventInfoArg > ().unwrap(),)); },
        ::protocol::PtcPlayerSyncArg::PROTOCOL_ID => { $session
        .notify(::evelyn_proto::PlayerSyncScNotify::from($ctx .get_arg:: <
        ::protocol::PtcPlayerSyncArg > ().unwrap(),)); },
        ::protocol::PtcEnterSceneArg::PROTOCOL_ID => { $session
        .notify(::evelyn_proto::EnterSceneScNotify::from($ctx .get_arg:: <
        ::protocol::PtcEnterSceneArg > ().unwrap(),)); }, _ => (), }
    };
}
