use std::collections::HashMap;

use qwer::{OctData, ProtocolID};

mod enums;
mod retcode;
pub use retcode::*;

pub mod util;

pub mod action_info;
pub mod dungeon_info;
pub mod event_graph_info;
pub mod item_info;
pub mod player_info;
pub mod quest_info;
pub mod scene_ext;
pub mod scene_info;
pub use enums::*;

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(1)]
pub struct PtcKeepAliveArg {}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(100)]
pub struct RpcPlayerLoginArg {
    pub platform: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcPlayerLoginRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default)]
pub struct PlayerBasicInfo {
    pub avatar_id: u32,
    pub last_enter_world_timestamp: i64,
    pub player_avatar_id: u32,
    pub main_city_avatar_id: u32,
    pub level: u32,
    pub nick_name: String,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(101)]
pub struct RpcGetPlayerBasicInfoArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetPlayerBasicInfoRet {
    pub retcode: Retcode,
    pub basic_info: PlayerBasicInfo,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(102)]
pub struct RpcGetAvatarDataArg {}

#[derive(OctData, Debug, Default)]
pub struct AvatarSkillInfo {
    pub skill_type: u32,
    pub level: u32,
}

#[derive(OctData, Debug, Default)]
pub struct AvatarInfo {
    pub ngfmenjlddl: u32,
    pub id: u32,
    pub level: u32,
    pub exp: u32,
    pub star: u32,
    pub rank: u32,
    pub first_get_time: i64,
    pub unlocked_talent_num: u32,
    pub talent_switch_list: Vec<bool>,
    pub skill_type_level: Vec<AvatarSkillInfo>,
    pub cur_weapon_uid: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcGetAvatarDataRet {
    pub retcode: Retcode,
    pub avatar_list: Vec<AvatarInfo>,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(103)]
pub struct RpcGetWeaponDataArg {}

#[derive(OctData, Debug, Default)]
pub struct WeaponInfo {
    pub id: u32,
    pub level: u32,
    pub exp: u32,
    pub star: u32,
    pub refine_level: u32,
    pub uid: u32,
    pub lock: bool,
}

#[derive(OctData, Debug, Default)]
pub struct RpcGetWeaponDataRet {
    pub retcode: Retcode,
    pub weapon_list: Vec<WeaponInfo>,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(104)]
pub struct RpcGetEquipDataArg {}

#[derive(OctData, Debug, Default)]
pub struct EquipInfo {
    pub id: u32,
    pub level: u32,
    pub exp: u32,
    pub star: u32,
    pub uid: u32,
    pub lock: bool,
}

#[derive(OctData, Debug, Default)]
pub struct RpcGetEquipDataRet {
    pub retcode: Retcode,
    pub equip_list: Vec<EquipInfo>,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(105)]
pub struct RpcGetResourceDataArg {}

#[derive(OctData, Debug, Default)]
pub struct ResourceInfo {
    pub id: u32,
    pub count: i32,
}

#[derive(OctData, Clone, Debug, Default)]
pub struct AutoRecoveryInfo {
    pub last_recovery_timestamp: i64,
    pub buy_times: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcGetResourceDataRet {
    pub retcode: Retcode,
    pub resource_list: Vec<ResourceInfo>,
    pub auto_recovery_info: HashMap<u32, AutoRecoveryInfo>,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(106)]
pub struct RpcGetQuestDataArg {
    pub quest_type: u32,
}

#[derive(OctData, Debug, Default)]
pub struct QuestCollection {
    pub quest_type: u32,
    pub quest_id_list: Vec<u32>,
}

#[derive(OctData, Debug, Default)]
pub struct QuestData {
    pub quest_collection_list: Vec<QuestCollection>,
}

#[derive(OctData, Debug, Default)]
pub struct RpcGetQuestDataRet {
    pub retcode: Retcode,
    pub quest_type: u32,
    pub quest_data: QuestData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(107)]
pub struct RpcGetArchiveDataArg {}

#[derive(OctData, Debug, Default)]
pub struct VideotapeInfo {
    pub archive_file_id: u32,
    pub finished: bool,
}

#[derive(OctData, Debug, Default)]
pub struct ArchiveData {
    pub hollow_archive_id_list: Vec<u32>,
    pub videotaps_info: Vec<VideotapeInfo>,
}

#[derive(OctData, Debug, Default)]
pub struct RpcGetArchiveDataRet {
    pub retcode: Retcode,
    pub archive_data: ArchiveData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(108)]
pub struct RpcGetHollowDataArg {}

#[derive(OctData, Debug, Default)]
pub struct HollowData {
    pub unlock_hollow_id_list: Vec<u32>,
}

#[derive(OctData, Debug, Default)]
pub struct RpcGetHollowDataRet {
    pub retcode: Retcode,
    pub hollow_data: HollowData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(109)]
pub struct RpcAbyssGetDataArg {}

#[derive(OctData, Debug, Default)]
pub struct AbyssInfo {}

#[derive(OctData, Debug, Default)]
pub struct RpcAbyssGetDataRet {
    pub retcode: Retcode,
    pub abyss_info: AbyssInfo,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(110)]
pub struct RpcGetBuddyDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetBuddyDataRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(111)]
pub struct RpcAbyssArpeggioGetDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcAbyssArpeggioGetDataRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(112)]
pub struct RpcGetServerTimestampArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetServerTimestampRet {
    pub retcode: Retcode,
    pub utc_offset: i32,
    pub timestamp: u64,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(113)]
pub struct RpcVideoGetInfoArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcVideoGetInfoRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(114)]
pub struct RpcGetAuthkeyArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetAuthkeyRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(115)]
pub struct RpcGetGachaDataArg {
    pub gacha_type: u32,
}

#[derive(OctData, Debug, Default)]
pub struct GachaData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetGachaDataRet {
    pub retcode: Retcode,
    pub gacha_type: u32,
    pub gacha_data: GachaData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(116)]
pub struct RpcGetCampIdleDataArg {}

#[derive(OctData, Debug, Default)]
pub struct CampIdleData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetCampIdleDataRet {
    pub retcode: Retcode,
    pub camp_idle_data: CampIdleData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(117)]
pub struct RpcSavePlayerSystemSettingArg {
    pub params: u32,
    pub r#type: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcSavePlayerSystemSettingRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(118)]
pub struct RpcGetRamenDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RamenData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetRamenDataRet {
    pub retcode: Retcode,
    pub ramen_data: RamenData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(119)]
pub struct RpcGetCafeDataArg {}

#[derive(OctData, Debug, Default)]
pub struct CafeData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetCafeDataRet {
    pub retcode: Retcode,
    pub cafe_data: CafeData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(120)]
pub struct RpcGetRewardBuffDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RewardBuffData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetRewardBuffDataRet {
    pub retcode: Retcode,
    pub data: RewardBuffData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(121)]
pub struct RpcGetPlayerMailsArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetPlayerMailsRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(122)]
pub struct RpcGetFairyDataArg {}

#[derive(OctData, Debug, Default)]
pub struct FairyData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetFairyDataRet {
    pub retcode: Retcode,
    pub data: FairyData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(123)]
pub struct RpcGetTipsInfoArg {}

#[derive(OctData, Debug, Default)]
pub struct TipsInfo {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetTipsInfoRet {
    pub retcode: Retcode,
    pub tips_info: TipsInfo,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(124)]
pub struct RpcGetClientSystemsDataArg {}

#[derive(OctData, Debug, Default)]
pub struct UnlockData {
    pub unlocked_list: Vec<i32>,
}

#[derive(OctData, Debug, Default)]
pub struct PostGirlItem {
    pub id: u32,
    pub unlock_time: i64,
}

#[derive(OctData, Debug, Default)]
pub struct PostGirlData {
    pub post_girl_item_list: Vec<PostGirlItem>,
    pub selected_post_girl_id_list: Vec<u32>,
    pub show_random_selected: bool,
}

#[derive(OctData, Debug, Default)]
pub struct ClientSystemsData {
    pub unlock_data: UnlockData,
    pub post_girl_data: PostGirlData,
}

#[derive(OctData, Debug, Default)]
pub struct RpcGetClientSystemsDataRet {
    pub retcode: Retcode,
    pub data: ClientSystemsData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(125)]
pub struct RpcGetPrivateMessageDataArg {}

#[derive(OctData, Debug, Default)]
pub struct PrivateMessageData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetPrivateMessageDataRet {
    pub retcode: Retcode,
    pub private_message_data: PrivateMessageData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(126)]
pub struct RpcGetCollectMapArg {}

#[derive(OctData, Debug, Default)]
pub struct CollectMap {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetCollectMapRet {
    pub retcode: Retcode,
    pub collect_map: CollectMap,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(127)]
pub struct RpcWorkbenchGetDataArg {}

#[derive(OctData, Debug, Default)]
pub struct WorkbenchData {}

#[derive(OctData, Debug, Default)]
pub struct RpcWorkbenchGetDataRet {
    pub retcode: Retcode,
    pub workbench_data: WorkbenchData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(128)]
pub struct RpcGetAbyssRewardDataArg {}

#[derive(OctData, Debug, Default)]
pub struct AbyssRewardData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetAbyssRewardDataRet {
    pub retcode: Retcode,
    pub abyss_reward_data: AbyssRewardData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(129)]
pub struct RpcGetVhsStoreDataArg {}

#[derive(OctData, Debug, Default)]
pub struct VhsStoreData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetVhsStoreDataRet {
    pub retcode: Retcode,
    pub data: VhsStoreData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(130)]
pub struct RpcGetActivityDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetActivityDataRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(131)]
pub struct RpcGetWebActivityDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetWebActivityDataRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(132)]
pub struct RpcGetEmbattlesDataArg {}

#[derive(OctData, Debug, Default)]
pub struct EmbattlesData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetEmbattlesDataRet {
    pub retcode: Retcode,
    pub embattles_data: EmbattlesData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(133)]
pub struct RpcGetNewsStandDataArg {}

#[derive(OctData, Debug, Default)]
pub struct NewsStandData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetNewsStandDataRet {
    pub retcode: Retcode,
    pub news_stand_data: NewsStandData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(134)]
pub struct RpcGetTrashbinHermitDataArg {}

#[derive(OctData, Debug, Default)]
pub struct TrashbinHermitData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetTrashbinHermitDataRet {
    pub retcode: Retcode,
    pub trashbin_hermit_data: TrashbinHermitData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(135)]
pub struct RpcGetMainCityRevivalDataArg {}

#[derive(OctData, Debug, Default)]
pub struct MainCityRevivalData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetMainCityRevivalDataRet {
    pub retcode: Retcode,
    pub main_city_revival_data: MainCityRevivalData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(136)]
pub struct RpcGetArcadeDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetArcadeDataRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(137)]
pub struct RpcGetBattlePassDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetBattlePassDataRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(138)]
pub struct RpcGetHadalZoneDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetHadalZoneDataRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(139)]
pub struct RpcGetBabelTowerDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetBabelTowerDataRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(140)]
pub struct RpcGetDailyChallengeDataArg {}

#[derive(OctData, Debug, Default)]
pub struct DailyChallengeData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetDailyChallengeDataRet {
    pub retcode: Retcode,
    pub data: DailyChallengeData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(141)]
pub struct RpcGetRoleCardDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RoleCardData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetRoleCardDataRet {
    pub retcode: Retcode,
    pub role_card_data: RoleCardData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(142)]
pub struct RpcGetChatEmojiListArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetChatEmojiListRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(143)]
pub struct RpcGetFriendListArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetFriendListRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(144)]
pub struct RpcGetCharacterQuestListArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetCharacterQuestListRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(145)]
pub struct RpcGetExplorationDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetExplorationDataRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(146)]
pub struct RpcGetFashionStoreDataArg {}

#[derive(OctData, Debug, Default)]
pub struct FashionStoreData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetFashionStoreDataRet {
    pub retcode: Retcode,
    pub data: FashionStoreData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(147)]
pub struct RpcGetShoppingMallInfoArg {}

#[derive(OctData, Debug, Default)]
pub struct ShoppingMallInfo {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetShoppingMallInfoRet {
    pub retcode: Retcode,
    pub shopping_mall_info: ShoppingMallInfo,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(148)]
pub struct RpcGetOnlineFriendsListArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetOnlineFriendsListRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(149)]
pub struct RpcEnterWorldArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcEnterWorldRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default)]
pub struct FreshSceneInfo {
    pub beginner_procedure_id: u32,
}

#[derive(OctData, Debug, Default, Clone)]
pub struct InteractInfo {
    pub interact_id: i32,
    pub name: String,
    pub participators: HashMap<u32, String>,
    pub scale_x: f64,
    pub scale_y: f64,
    pub scale_z: f64,
    pub scale_w: f64,
    pub scale_r: f64,
    pub interact_target_list: Vec<i32>,
}

#[derive(OctData, Debug, Default)]
pub struct SceneUnitProtocolInfo {
    pub npc_id: u32,
    pub is_interactable: bool,
    pub interacts_info: HashMap<u32, InteractInfo>,
}

#[derive(OctData, Debug, Default)]
pub struct HallSceneInfo {
    pub section_id: u32,
    pub player_avatar_id: u32,
    pub main_city_avatar_id: u32,
    pub transform_id: String,
    pub bgm_id: u32,
    pub day_of_week: u32,
    pub time_of_day: u32,
    pub camera_x: u32,
    pub camera_y: u32,
    pub position: Option<Transform>,
    pub scene_unit_list: Vec<SceneUnitProtocolInfo>,
    pub main_city_objects_state: HashMap<i32, i32>,
}

#[derive(OctData, Debug, Default)]
pub struct LevelPerformInfo {
    pub time: String,
    pub weather: String,
}

#[derive(OctData, Debug, Default)]
pub struct LevelRewardInfo {}

#[derive(OctData, Debug, Default)]
pub struct FightSceneInfo {
    pub end_hollow: bool,
    pub level_perform_info: LevelPerformInfo,
    pub level_reward_info: LevelRewardInfo,
}

#[derive(OctData, Debug, Default)]
pub struct SceneInfo {
    pub scene_type: u32,
    pub event_id: u32,
    pub local_play_type: u32,
    pub hall_scene_info: Option<HallSceneInfo>,
    pub fresh_scene_info: Option<FreshSceneInfo>,
    pub fight_scene_info: Option<FightSceneInfo>,
}

#[derive(OctData, Debug, Default)]
pub struct DungeonEquipInfo {}

#[derive(OctData, Debug, Default)]
pub struct AvatarUnitInfo {
    pub avatar_id: u32,
}

#[derive(OctData, Debug, Default)]
pub struct DungeonInfo {
    pub quest_id: u32,
    pub dungeon_equip_info: DungeonEquipInfo,
    pub avatar_list: Vec<AvatarUnitInfo>,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(150)]
pub struct PtcEnterSceneArg {
    pub scene_info: SceneInfo,
    pub dungeon_info: Option<DungeonInfo>,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(151)]
pub struct RpcSceneTransitionArg {
    pub section_id: u32,
    pub reason: String,
}

#[derive(OctData, Debug, Default)]
pub struct RpcSceneTransitionRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(152)]
pub struct RpcEnterSectionCompleteArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcEnterSectionCompleteRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(153)]
pub struct RpcGetMonthCardRewardListArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetMonthCardRewardListRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(154)]
pub struct RpcGetDisplayCaseDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetDisplayCaseDataRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default)]
pub struct Transform {
    pub position: Vec<f64>,
    pub rotation: Vec<f64>,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(155)]
pub struct RpcSavePosInMainCityArg {
    pub position: Transform,
    pub section_id: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcSavePosInMainCityRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(156)]
pub struct RpcPlayerOperationArg {
    pub system: u32,
    pub operator: u32,
    pub param: i32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcPlayerOperationRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(157)]
pub struct RpcReportUiLayoutPlatformArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcReportUiLayoutPlatformRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(158)]
pub struct RpcPlayerTransactionArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcPlayerTransactionRet {
    pub retcode: Retcode,
    pub transaction: String,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(159)]
pub struct RpcRechargeGetItemListArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcRechargeGetItemListRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(160)]
pub struct PtcAddAvatarArg {
    pub avatar_id: u32,
    pub perform_type: i32,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(161)]
pub struct RpcGetWishlistDataArg {}

#[derive(OctData, Debug, Default)]
pub struct SkillWishlistPlan {
    pub plan_type: i32,
    pub wish_skill_id_list: Vec<u32>,
}

#[derive(OctData, Debug, Default)]
pub struct AvatarWishlistPlan {
    pub plan_type: i32,
    pub avatar_plan_param1: u32,
    pub avatar_plan_param2: u32,
}

#[derive(OctData, Debug, Default)]
pub struct EquipWishlistPlan {
    pub plan_type: i32,
    pub wish_weapon_id_list: Vec<u32>,
    pub wish_equip_id_list: Vec<u32>,
}

#[derive(OctData, Debug, Default)]
pub struct WishlistPlanInfo {
    pub avatar_id: u32,
    pub avatar_wishlist_plan: Option<AvatarWishlistPlan>,
    pub equip_wishlist_plan: Option<EquipWishlistPlan>,
    pub skill_wishlist_plan: Option<SkillWishlistPlan>,
}

#[derive(OctData, Debug, Default)]
pub struct RpcGetWishlistDataRet {
    pub retcode: Retcode,
    pub wishlist_plan_list: Vec<WishlistPlanInfo>,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(162)]
pub struct RpcGetMiniscapeEntrustDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetMiniscapeEntrustDataRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(163)]
pub struct RpcGetJourneyDataArg {}

#[derive(OctData, Debug, Default)]
pub struct JourneyData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetJourneyDataRet {
    pub retcode: Retcode,
    pub data: JourneyData,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(164)]
pub struct RpcGetPhotoWallDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetPhotoWallDataRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(165)]
pub struct RpcModMainCityAvatarArg {
    pub avatar_id: u32,
    pub player_avatar_id: u32,
    pub main_city_avatar_id: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcModMainCityAvatarRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default)]
pub struct AvatarSync {
    pub avatar_list: Vec<AvatarInfo>,
}

#[derive(OctData, Debug, Default)]
pub struct ItemSync {
    pub weapon_list: Vec<WeaponInfo>,
    pub equip_list: Vec<EquipInfo>,
    pub resource_list: Vec<ResourceInfo>,
    pub auto_recovery_info: HashMap<u32, AutoRecoveryInfo>,
}

#[derive(OctData, Debug, Default)]
pub struct PostGirlSyncData {
    pub show_random_selected: bool,
    pub selected_post_girl_id_list: Vec<u32>,
    pub add_post_girl_item_list: Vec<PostGirlItem>,
}

#[derive(OctData, Debug, Default)]
pub struct ClientSystemsSync {
    pub post_girl_data: Option<PostGirlSyncData>,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(500)]
pub struct PtcPlayerSyncArg {
    pub basic_info: Option<PlayerBasicInfo>,
    pub avatar: Option<AvatarSync>,
    pub item: Option<ItemSync>,
    pub client_systems: Option<ClientSystemsSync>,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(501)]
pub struct PtcHallRefreshArg {
    pub section_id: u32,
    pub player_avatar_id: u32,
    pub main_city_avatar_id: u32,
    pub transform_id: String,
    pub bgm_id: u32,
    pub day_of_week: u32,
    pub time_of_day: u32,
    pub camera_x: u32,
    pub camera_y: u32,
    pub position: Transform,
    pub refresh_immediately: bool,
    pub scene_unit_list: Vec<SceneUnitProtocolInfo>,
    pub main_city_objects_state: HashMap<i32, i32>,
}

#[derive(OctData, Debug, Default)]
pub struct ActionInfo {
    pub action_type: i32,
    pub body: Vec<u8>,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(502)]
pub struct PtcSyncEventInfoArg {
    pub owner_id: u32,
    pub npc_interaction: String,
    pub tag: u32,
    pub owner_type: i32,
    pub action_list: Vec<ActionInfo>,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(503)]
pub struct PtcUpdateEventGraphArg {
    pub event_graph_uid: u32,
    pub event_graph_owner_uid: u32,
    pub tag: u32,
    pub npc_interaction: String,
    pub owner_type: i32,
    pub is_event_success: bool,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(166)]
pub struct RpcModTimeArg {
    pub time_period: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcModTimeRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(167)]
pub struct RpcInteractWithClientEntityArg {
    pub interaction: i32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcInteractWithClientEntityRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(168)]
pub struct RpcInteractWithUnitArg {
    pub r#type: i32,
    pub interaction: i32,
    pub npc_tag_id: i32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcInteractWithUnitRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(169)]
pub struct RpcRunEventGraphArg {
    pub owner_type: i32,
    pub owner_id: u32,
    pub section_id: u32,
    pub tag: u32,
    pub event_graph_uid: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcRunEventGraphRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(170)]
pub struct RpcEnterSectionArg {
    pub section_id: u32,
    pub transform_id: String,
}

#[derive(OctData, Debug, Default)]
pub struct RpcEnterSectionRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(171)]
pub struct RpcRefreshSectionArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcRefreshSectionRet {
    pub retcode: Retcode,
    pub refresh_status: u32,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(172)]
pub struct RpcCheckYorozuyaInfoRefreshArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcCheckYorozuyaInfoRefreshRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(173)]
pub struct RpcBeginTrainingCourseBattleArg {
    pub avatar_id_list: Vec<u32>,
    pub buddy_id: u32,
    pub quest_id: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcBeginTrainingCourseBattleRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(174)]
pub struct RpcReportEmbattleInfoArg {
    pub avatar_list: Vec<i32>,
}

#[derive(OctData, Debug, Default)]
pub struct RpcReportEmbattleInfoRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(175)]
pub struct RpcBattleReportArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcBattleReportRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(176)]
pub struct RpcEndBattleArg {
    // TODO: LogBattleStatistics
}

#[derive(OctData, Debug, Default)]
pub struct FightSettle {}

#[derive(OctData, Debug, Default)]
pub struct RpcEndBattleRet {
    pub retcode: Retcode,
    pub fight_settle: FightSettle,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(177)]
pub struct RpcLeaveCurSceneArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcLeaveCurSceneRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(178)]
pub struct RpcGetPlayerNetworkDataArg {}

#[derive(OctData, Debug, Default)]
pub struct PlayerNetworkData {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetPlayerNetworkDataRet {
    pub retcode: Retcode,
    pub player_network_data: Option<PlayerNetworkData>,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(179)]
pub struct RpcWeaponDressArg {
    pub avatar_id: u32,
    pub weapon_uid: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcWeaponDressRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(180)]
pub struct RpcWeaponUnDressArg {
    pub avatar_id: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcWeaponUnDressRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(181)]
pub struct RpcGetRidusGotBooDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetRidusGotBooDataRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(182)]
pub struct RpcGetFishingContestDataArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetFishingContestDataRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(183)]
pub struct RpcGetRedDotListArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcGetRedDotListRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(184)]
pub struct RpcGetAvatarRecommendEquipArg {
    pub avatar_id: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcGetAvatarRecommendEquipRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(185)]
pub struct RpcPostEnterWorldArg {}

#[derive(OctData, Debug, Default)]
pub struct RpcPostEnterWorldRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(186)]
pub struct RpcSetLanguageArg {
    pub language: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcSetLanguageRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(187)]
pub struct RpcSelectPostGirlArg {
    pub show_random_selected: bool,
    pub new_selected_post_girl_id_list: Vec<u32>,
}

#[derive(OctData, Debug, Default)]
pub struct RpcSelectPostGirlRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(188)]
pub struct RpcBeginArchiveBattleQuestArg {
    pub quest_id: u32,
    pub is_story: bool,
    pub avatar_id_list: Vec<u32>,
    pub buddy_id: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcBeginArchiveBattleQuestRet {
    pub retcode: Retcode,
    pub quest_id: u32,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(189)]
pub struct RpcPerformTriggerArg {
    pub perform_id: i32,
    pub perform_type: i32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcPerformTriggerRet {
    pub retcode: Retcode,
    pub perform_uid: i64,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(190)]
pub struct RpcPerformJumpArg {
    pub perform_id: i32,
    pub perform_type: i32,
    pub perform_uid: i64,
}

#[derive(OctData, Debug, Default)]
pub struct RpcPerformJumpRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(191)]
pub struct RpcPerformEndArg {
    pub perform_id: i32,
    pub perform_type: i32,
    pub perform_uid: i64,
}

#[derive(OctData, Debug, Default)]
pub struct RpcPerformEndRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(192)]
pub struct RpcEndNewbieArg {
    pub group_id: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcEndNewbieRet {
    pub retcode: Retcode,
}

#[derive(OctData, Debug, Default, ProtocolID)]
#[id(193)]
pub struct RpcFinishArchiveQuestArg {
    pub quest_id: u32,
}

#[derive(OctData, Debug, Default)]
pub struct RpcFinishArchiveQuestRet {
    pub retcode: Retcode,
    pub quest_id: u32,
}
