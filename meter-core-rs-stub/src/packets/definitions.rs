use serde::{Deserialize, Serialize};

use crate::{CharacterId, ClassId, EntityId, GearLevel, PartyInstanceId, RaidInstanceId, SkillEffectId, SkillId, StatusEffectInstanceId};

use super::structures::*;

macro_rules! impl_new_default {
    ($struct_name:ident) => {
        impl $struct_name {
            pub fn new(data: &[u8]) -> anyhow::Result<Self> {
                Ok(serde_json::from_slice(data).unwrap())
            }
        }
    };
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTCounterAttackNotify {
    pub source_id: u64
}

impl_new_default!(PKTCounterAttackNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTDeathNotify {
    pub target_id: u64
}

impl_new_default!(PKTDeathNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTIdentityGaugeChangeNotify {
    pub player_id: EntityId,
    pub identity_gauge1: u32,
    pub identity_gauge2: u32,
    pub identity_gauge3: u32
}

impl_new_default!(PKTIdentityGaugeChangeNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTInitEnv {
    pub player_id: EntityId
}

impl_new_default!(PKTInitEnv);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTInitPC {
    pub player_id: EntityId,
    pub name: String,
    pub character_id: CharacterId,
    pub class_id: CharacterId,
    pub gear_level: GearLevel,
    pub stat_pairs: Vec<StatPair>,
    pub status_effect_datas: Vec<StatusEffectData>,
}

impl_new_default!(PKTInitPC);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTNewNpc {
    pub npc_struct: NpcStruct
}

impl_new_default!(PKTNewNpc);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTNewNpcSummon {
    pub owner_id: EntityId,
    pub npc_struct: NpcStruct
}

impl_new_default!(PKTNewNpcSummon);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTNewProjectileInner {
    pub projectile_id: EntityId,
    pub owner_id: EntityId,
    pub skill_id: SkillId,
    pub skill_effect: SkillEffectId,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTNewProjectile {
    pub projectile_info: PKTNewProjectileInner
}

impl_new_default!(PKTNewProjectile);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTNewTrapInner {
    pub object_id: EntityId,
    pub owner_id: EntityId,
    pub skill_id: SkillId,
    pub skill_effect: SkillEffectId
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTNewTrap {
    pub trap_struct: PKTNewTrapInner
}

impl_new_default!(PKTNewTrap);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTRaidBegin {
    pub raid_id: u32,
}

impl_new_default!(PKTRaidBegin);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTRemoveObjectInner {
    pub object_id: EntityId
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTRemoveObject {
    pub unpublished_objects: Vec<PKTRemoveObjectInner>
}

impl_new_default!(PKTRemoveObject);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTSkillCastNotify {
    pub source_id: EntityId,
    pub skill_id: SkillId,
}

impl_new_default!(PKTSkillCastNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub struct TripodIndex {
    pub first: u8,
    pub second: u8,
    pub third: u8,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub struct TripodLevel {
    pub first: u16,
    pub second: u16,
    pub third: u16,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTSkillStartNotifyInner {
    pub tripod_index: Option<TripodIndex>,
    pub tripod_level: Option<TripodLevel>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTSkillStartNotify {
    pub source_id: EntityId,
    pub skill_id: SkillId,
    pub skill_option_data: PKTSkillStartNotifyInner,
}

impl_new_default!(PKTSkillStartNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTSkillDamageAbnormalMoveNotifyInner {
    pub skill_damage_event: SkillDamageEvent
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTSkillDamageAbnormalMoveNotify {
    pub source_id: EntityId,
    pub skill_damage_abnormal_move_events: Vec<PKTSkillDamageAbnormalMoveNotifyInner>,
    pub skill_id: SkillId,
    pub skill_effect_id: SkillEffectId,
}

impl_new_default!(PKTSkillDamageAbnormalMoveNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTSkillDamageNotify {
    pub source_id: EntityId,
    pub skill_damage_events: Vec<SkillDamageEvent>,
    pub skill_id: SkillId,
    pub skill_effect_id: Option<SkillEffectId>,
}

impl_new_default!(PKTSkillDamageNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTPartyInfoInner {
    pub name: String,
    pub class_id: ClassId,
    pub character_id: CharacterId,
    pub gear_level: GearLevel,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTPartyInfo {
    pub party_instance_id: PartyInstanceId,
    pub raid_instance_id: RaidInstanceId,
    pub party_member_datas: Vec<PKTPartyInfoInner>
}

impl_new_default!(PKTPartyInfo);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTPartyLeaveResult {
    pub party_instance_id: PartyInstanceId,
    pub name: String
}

impl_new_default!(PKTPartyLeaveResult);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTPartyStatusEffectAddNotify {
    pub character_id: u64,
    pub status_effect_datas: Vec<StatusEffectData>
}

impl_new_default!(PKTPartyStatusEffectAddNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTPartyStatusEffectRemoveNotify {
    pub character_id: CharacterId,
    pub status_effect_instance_ids: Vec<StatusEffectInstanceId>,
    pub reason: u8
}

impl_new_default!(PKTPartyStatusEffectRemoveNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTPartyStatusEffectResultNotify {
    pub raid_instance_id: RaidInstanceId,
    pub party_instance_id: PartyInstanceId,
    pub character_id: CharacterId
}

impl_new_default!(PKTPartyStatusEffectResultNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTStatusEffectAddNotify {
    pub object_id: EntityId,
    pub status_effect_data: StatusEffectData
}

impl_new_default!(PKTStatusEffectAddNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTStatusEffectRemoveNotify {
    pub object_id: EntityId,
    pub character_id: CharacterId,
    pub status_effect_instance_ids: Vec<StatusEffectInstanceId>,
    pub reason: u8
}

impl_new_default!(PKTStatusEffectRemoveNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTTriggerStartNotify {
    pub signal: u32,
}

impl_new_default!(PKTTriggerStartNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTZoneMemberLoadStatusNotify {
    pub zone_id: u32,
    pub zone_level: u32
}

impl_new_default!(PKTZoneMemberLoadStatusNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTZoneObjectUnpublishNotify {
    pub object_id: u64
}

impl_new_default!(PKTZoneObjectUnpublishNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTStatusEffectSyncDataNotify {
    pub object_id: EntityId,
    pub status_effect_instance_id: StatusEffectInstanceId,
    pub character_id: CharacterId,
    pub value: u64,
}

impl_new_default!(PKTStatusEffectSyncDataNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTTroopMemberUpdateMinNotify {
    pub character_id: u64,
    pub cur_hp: i64,
    pub max_hp: i64,
    pub status_effect_datas: Vec<StatusEffectData>,
}

impl_new_default!(PKTTroopMemberUpdateMinNotify);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTNewTransit {
    pub channel_id: u32
}

impl_new_default!(PKTNewTransit);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTNewPCInner {
    pub player_id: EntityId,
    pub name: String,
    pub class_id: ClassId,
    pub max_item_level: GearLevel,
    pub character_id: CharacterId,
    pub stat_pairs: Vec<StatPair>,
    pub equip_item_datas: Vec<EquipItemData>,
    pub status_effect_datas: Vec<StatusEffectData>
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PKTNewPC {
    pub pc_struct: PKTNewPCInner
}

impl_new_default!(PKTNewPC);