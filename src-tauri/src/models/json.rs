use hashbrown::HashMap;
use once_cell::sync::Lazy;

use super::*;

pub static NPC_DATA: Lazy<HashMap<u32, Npc>> = Lazy::new(|| {
    let json_data = include_bytes!("../../meter-data/Npc.json");
    serde_json::from_slice(json_data).unwrap()
});

pub static SKILL_DATA: Lazy<HashMap<u32, SkillData>> = Lazy::new(|| {
    let json_data = include_bytes!("../../meter-data/Skill.json");
    serde_json::from_slice(json_data).unwrap()
});

pub static SKILL_EFFECT_DATA: Lazy<HashMap<u32, SkillEffectData>> = Lazy::new(|| {
    let json_data = include_bytes!("../../meter-data/SkillEffect.json");
    serde_json::from_slice(json_data).unwrap()
});

pub static SKILL_BUFF_DATA: Lazy<HashMap<u32, SkillBuffData>> = Lazy::new(|| {
    let json_data = include_bytes!("../../meter-data/SkillBuff.json");
    serde_json::from_slice(json_data).unwrap()
});

pub static COMBAT_EFFECT_DATA: Lazy<HashMap<i32, CombatEffectData>> = Lazy::new(|| {
    let json_data = include_bytes!("../../meter-data/CombatEffect.json");
    serde_json::from_slice(json_data).unwrap()
});

pub static ENGRAVING_DATA: Lazy<HashMap<u32, EngravingData>> = Lazy::new(|| {
    let json_data = include_bytes!("../../meter-data/Ability.json");
    serde_json::from_slice(json_data).unwrap()
});

pub static ESTHER_DATA: Lazy<Vec<Esther>> = Lazy::new(|| {
    let json_data = include_bytes!("../../meter-data/Esther.json");
    serde_json::from_slice(json_data).unwrap()
});

pub static VALID_ZONES: Lazy<HashSet<u32>> = Lazy::new(|| {
    let valid_zones = [
        30801, 30802, 30803, 30804, 30805, 30806, 30807, 30835, 37001, 37002, 37003, 37011,
        37012, 37021, 37022, 37031, 37032, 37041, 37042, 37051, 37061, 37071, 37072, 37081,
        37091, 37092, 37093, 37094, 37101, 37102, 37111, 37112, 37121, 37122, 37123, 37124,
        308010, 308011, 308012, 308014, 308015, 308016, 308017, 308018, 308019, 308020, 308021,
        308022, 308023, 308024, 308025, 308026, 308027, 308028, 308029, 308030, 308037, 308039,
        308040, 308041, 308042, 308043, 308044, 308239, 308339, 308410, 308411, 308412, 308414,
        308415, 308416, 308417, 308418, 308419, 308420, 308421, 308422, 308423, 308424, 308425,
        308426, 308428, 308429, 308430, 308437, 309020, 30865, 30866
    ];

    valid_zones.iter().cloned().collect()
});

pub static STAT_TYPE_MAP: Lazy<HashMap<&'static str, u32>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("none", 0);
    map.insert("hp", 1);
    map.insert("mp", 2);
    map.insert("str", 3);
    map.insert("agi", 4);
    map.insert("int", 5);
    map.insert("con", 6);
    map.insert("str_x", 7);
    map.insert("agi_x", 8);
    map.insert("int_x", 9);
    map.insert("con_x", 10);
    map.insert("criticalhit", 15);
    map.insert("specialty", 16);
    map.insert("oppression", 17);
    map.insert("rapidity", 18);
    map.insert("endurance", 19);
    map.insert("mastery", 20);
    map.insert("criticalhit_x", 21);
    map.insert("specialty_x", 22);
    map.insert("oppression_x", 23);
    map.insert("rapidity_x", 24);
    map.insert("endurance_x", 25);
    map.insert("mastery_x", 26);
    map.insert("max_hp", 27);
    map.insert("max_mp", 28);
    map.insert("max_hp_x", 29);
    map.insert("max_mp_x", 30);
    map.insert("max_hp_x_x", 31);
    map.insert("max_mp_x_x", 32);
    map.insert("normal_hp_recovery", 33);
    map.insert("combat_hp_recovery", 34);
    map.insert("normal_hp_recovery_rate", 35);
    map.insert("combat_hp_recovery_rate", 36);
    map.insert("normal_mp_recovery", 37);
    map.insert("combat_mp_recovery", 38);
    map.insert("normal_mp_recovery_rate", 39);
    map.insert("combat_mp_recovery_rate", 40);
    map.insert("self_recovery_rate", 41);
    map.insert("drain_hp_dam_rate", 42);
    map.insert("drain_mp_dam_rate", 43);
    map.insert("dam_reflection_rate", 44);
    map.insert("char_attack_dam", 47);
    map.insert("skill_effect_dam_addend", 48);
    map.insert("attack_power_rate", 49);
    map.insert("skill_damage_rate", 50);
    map.insert("attack_power_rate_x", 51);
    map.insert("skill_damage_rate_x", 52);
    map.insert("cooldown_reduction", 53);
    map.insert("paralyzation_point_rate", 54);
    map.insert("def", 55);
    map.insert("res", 56);
    map.insert("def_x", 57);
    map.insert("res_x", 58);
    map.insert("def_x_x", 59);
    map.insert("res_x_x", 60);
    map.insert("def_pen_rate", 67);
    map.insert("res_pen_rate", 68);
    map.insert("physical_inc_rate", 69);
    map.insert("magical_inc_rate", 70);
    map.insert("self_shield_rate", 71);
    map.insert("hit_rate", 72);
    map.insert("dodge_rate", 73);
    map.insert("critical_hit_rate", 74);
    map.insert("critical_res_rate", 75);
    map.insert("critical_dam_rate", 76);
    map.insert("attack_speed", 77);
    map.insert("attack_speed_rate", 78);
    map.insert("move_speed", 79);
    map.insert("move_speed_rate", 80);
    map.insert("prop_move_speed", 81);
    map.insert("prop_move_speed_rate", 82);
    map.insert("vehicle_move_speed", 83);
    map.insert("vehicle_move_speed_rate", 84);
    map.insert("ship_move_speed", 85);
    map.insert("ship_move_speed_rate", 86);
    map.insert("fire_dam_rate", 87);
    map.insert("ice_dam_rate", 88);
    map.insert("electricity_dam_rate", 89);
    map.insert("earth_dam_rate", 91);
    map.insert("dark_dam_rate", 92);
    map.insert("holy_dam_rate", 93);
    map.insert("elements_dam_rate", 94);
    map.insert("fire_res_rate", 95);
    map.insert("ice_res_rate", 96);
    map.insert("electricity_res_rate", 97);
    map.insert("earth_res_rate", 99);
    map.insert("dark_res_rate", 100);
    map.insert("holy_res_rate", 101);
    map.insert("elements_res_rate", 102);
    map.insert("self_cc_time_rate", 105);
    map.insert("enemy_cc_time_rate", 106);
    map.insert("identity_value1", 107);
    map.insert("identity_value2", 108);
    map.insert("identity_value3", 109);
    map.insert("awakening_dam_rate", 110);
    map.insert("item_drop_rate", 111);
    map.insert("gold_rate", 112);
    map.insert("exp_rate", 113);
    map.insert("attack_power_addend", 123);
    map.insert("npc_species_humanoid_dam_rate", 125);
    map.insert("npc_species_devil_dam_rate", 126);
    map.insert("npc_species_substance_dam_rate", 127);
    map.insert("npc_species_undead_dam_rate", 128);
    map.insert("npc_species_plant_dam_rate", 129);
    map.insert("npc_species_insect_dam_rate", 130);
    map.insert("npc_species_spirit_dam_rate", 131);
    map.insert("npc_species_wild_beast_dam_rate", 132);
    map.insert("npc_species_mechanic_dam_rate", 133);
    map.insert("npc_species_ancient_dam_rate", 134);
    map.insert("npc_species_god_dam_rate", 135);
    map.insert("npc_species_archfiend_dam_rate", 136);
    map.insert("vitality", 137);
    map.insert("ship_booter_speed", 138);
    map.insert("ship_wreck_speed_rate", 139);
    map.insert("island_speed_rate", 140);
    map.insert("attack_power_sub_rate_1", 141);
    map.insert("attack_power_sub_rate_2", 142);
    map.insert("physical_inc_sub_rate_1", 143);
    map.insert("physical_inc_sub_rate_2", 144);
    map.insert("magical_inc_sub_rate_1", 145);
    map.insert("magical_inc_sub_rate_2", 146);
    map.insert("skill_damage_sub_rate_1", 147);
    map.insert("skill_damage_sub_rate_2", 148);
    map.insert("resource_recovery_rate", 149);
    map.insert("weapon_dam", 151);
    map
});

pub static STAT_TYPE_MAP_TRA: Lazy<HashMap<u32, &'static str>> = Lazy::new(|| {
    STAT_TYPE_MAP.iter().map(|(k, v)| (*v, *k)).collect()
});

pub static IDENTITY_CATEGORY: Lazy<HashMap<&'static str, i32>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("none", 0);
    map.insert("berserker_normal", 1);
    map.insert("berserker_rush", 2);
    map.insert("warlord_normal", 3);
    map.insert("warlord_shield_of_battlefield", 4);
    map.insert("destroyer_normal", 5);
    map.insert("destroyer_focus", 6);
    map.insert("destroyer_release", 7);
    map.insert("battle_master_normal", 8);
    map.insert("battle_master_bubble", 9);
    map.insert("infighter_normal", 10);
    map.insert("infighter_vigor", 11);
    map.insert("infighter_shock", 12);
    map.insert("forcemaster_normal", 13);
    map.insert("forcemaster_soul", 14);
    map.insert("lance_master_normal", 15);
    map.insert("lance_master_wild", 16);
    map.insert("lance_master_focus", 17);
    map.insert("devil_hunter_normal", 18);
    map.insert("devil_hunter_pistol", 19);
    map.insert("devil_hunter_shotgun", 20);
    map.insert("devil_hunter_rifle", 21);
    map.insert("blaster_normal", 22);
    map.insert("blaster_cannon", 23);
    map.insert("hawkeye_normal", 24);
    map.insert("hawkeye_summon", 25);
    map.insert("summoner_normal", 26);
    map.insert("summoner_ancient", 27);
    map.insert("arcana_normal", 28);
    map.insert("arcana_stack", 29);
    map.insert("arcana_ruin", 30);
    map.insert("arcana_card", 31);
    map.insert("bard_normal", 32);
    map.insert("bard_serenade", 33);
    map.insert("blade_burst", 34);
    map.insert("holyknight_normal", 35);
    map.insert("holyknight_holy", 36);
    map.insert("holyknight_retribution", 37);
    map.insert("demonic_normal", 38);
    map.insert("demonic_capture", 39);
    map.insert("demonic_demon", 40);
    map.insert("warlord_lance", 41);
    map.insert("reaper_normal", 42);
    map.insert("reaper_dagger", 43);
    map.insert("reaper_shadow", 44);
    map.insert("reaper_swoop", 45);
    map.insert("scouter_scout", 46);
    map.insert("scouter_drone", 47);
    map.insert("scouter_hyper_sync", 48);
    map.insert("scouter_fusion", 49);
    map.insert("blade_normal", 50);
    map.insert("elemental_master_normal", 51);
    map.insert("elemental_master_fire", 52);
    map.insert("elemental_master_electricity", 53);
    map.insert("elemental_master_ice", 54);
    map.insert("yinyangshi_normal", 55);
    map.insert("yinyangshi_yin", 56);
    map.insert("yinyangshi_yang", 57);
    map.insert("weather_artist_weapon", 58);
    map.insert("weather_artist_weather", 59);
    map.insert("summoner_summon", 60);
    map.insert("soul_eater_hollow", 61);
    map.insert("soul_eater_killer", 62);
    map.insert("soul_eater_death", 63);
    map
});

pub static NPC_GRADE: Lazy<HashMap<&'static str, i32>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("none", 0);
    map.insert("underling", 1);
    map.insert("normal", 2);
    map.insert("elite", 3);
    map.insert("named", 4);
    map.insert("seed", 5);
    map.insert("boss", 6);
    map.insert("raid", 7);
    map.insert("lucky", 8);
    map.insert("epic_raid", 9);
    map.insert("commander", 10);
    map
});
