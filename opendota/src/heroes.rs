use std::collections::HashMap;

use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct Hero {
    name: String,
    id: i64,
    localized_name: String,
}
pub static HEROS: Lazy<Mutex<HashMap<i64, String>>> = Lazy::new(|| {
    let data = json!([
        {
            "name": "npc_dota_hero_antimage",
            "id": 1,
            "localized_name": "敌法师"
        },
        {
            "name": "npc_dota_hero_axe",
            "id": 2,
            "localized_name": "斧王"
        },
        {
            "name": "npc_dota_hero_bane",
            "id": 3,
            "localized_name": "祸乱之源"
        },
        {
            "name": "npc_dota_hero_bloodseeker",
            "id": 4,
            "localized_name": "血魔"
        },
        {
            "name": "npc_dota_hero_crystal_maiden",
            "id": 5,
            "localized_name": "水晶室女"
        },
        {
            "name": "npc_dota_hero_drow_ranger",
            "id": 6,
            "localized_name": "卓尔游侠"
        },
        {
            "name": "npc_dota_hero_earthshaker",
            "id": 7,
            "localized_name": "撼地者"
        },
        {
            "name": "npc_dota_hero_juggernaut",
            "id": 8,
            "localized_name": "主宰"
        },
        {
            "name": "npc_dota_hero_mirana",
            "id": 9,
            "localized_name": "米拉娜"
        },
        {
            "name": "npc_dota_hero_nevermore",
            "id": 11,
            "localized_name": "影魔"
        },
        {
            "name": "npc_dota_hero_morphling",
            "id": 10,
            "localized_name": "变体精灵"
        },
        {
            "name": "npc_dota_hero_phantom_lancer",
            "id": 12,
            "localized_name": "幻影长矛手"
        },
        {
            "name": "npc_dota_hero_puck",
            "id": 13,
            "localized_name": "帕克"
        },
        {
            "name": "npc_dota_hero_pudge",
            "id": 14,
            "localized_name": "帕吉"
        },
        {
            "name": "npc_dota_hero_razor",
            "id": 15,
            "localized_name": "雷泽"
        },
        {
            "name": "npc_dota_hero_sand_king",
            "id": 16,
            "localized_name": "沙王"
        },
        {
            "name": "npc_dota_hero_storm_spirit",
            "id": 17,
            "localized_name": "风暴之灵"
        },
        {
            "name": "npc_dota_hero_sven",
            "id": 18,
            "localized_name": "斯温"
        },
        {
            "name": "npc_dota_hero_tiny",
            "id": 19,
            "localized_name": "小小"
        },
        {
            "name": "npc_dota_hero_vengefulspirit",
            "id": 20,
            "localized_name": "复仇之魂"
        },
        {
            "name": "npc_dota_hero_windrunner",
            "id": 21,
            "localized_name": "风行者"
        },
        {
            "name": "npc_dota_hero_zuus",
            "id": 22,
            "localized_name": "宙斯"
        },
        {
            "name": "npc_dota_hero_kunkka",
            "id": 23,
            "localized_name": "昆卡"
        },
        {
            "name": "npc_dota_hero_lina",
            "id": 25,
            "localized_name": "莉娜"
        },
        {
            "name": "npc_dota_hero_lich",
            "id": 31,
            "localized_name": "巫妖"
        },
        {
            "name": "npc_dota_hero_lion",
            "id": 26,
            "localized_name": "莱恩"
        },
        {
            "name": "npc_dota_hero_shadow_shaman",
            "id": 27,
            "localized_name": "暗影萨满"
        },
        {
            "name": "npc_dota_hero_slardar",
            "id": 28,
            "localized_name": "斯拉达"
        },
        {
            "name": "npc_dota_hero_tidehunter",
            "id": 29,
            "localized_name": "潮汐猎人"
        },
        {
            "name": "npc_dota_hero_witch_doctor",
            "id": 30,
            "localized_name": "巫医"
        },
        {
            "name": "npc_dota_hero_riki",
            "id": 32,
            "localized_name": "力丸"
        },
        {
            "name": "npc_dota_hero_enigma",
            "id": 33,
            "localized_name": "谜团"
        },
        {
            "name": "npc_dota_hero_tinker",
            "id": 34,
            "localized_name": "修补匠"
        },
        {
            "name": "npc_dota_hero_sniper",
            "id": 35,
            "localized_name": "狙击手"
        },
        {
            "name": "npc_dota_hero_necrolyte",
            "id": 36,
            "localized_name": "瘟疫法师"
        },
        {
            "name": "npc_dota_hero_warlock",
            "id": 37,
            "localized_name": "术士"
        },
        {
            "name": "npc_dota_hero_beastmaster",
            "id": 38,
            "localized_name": "兽王"
        },
        {
            "name": "npc_dota_hero_queenofpain",
            "id": 39,
            "localized_name": "痛苦女王"
        },
        {
            "name": "npc_dota_hero_venomancer",
            "id": 40,
            "localized_name": "剧毒术士"
        },
        {
            "name": "npc_dota_hero_faceless_void",
            "id": 41,
            "localized_name": "虚空假面"
        },
        {
            "name": "npc_dota_hero_skeleton_king",
            "id": 42,
            "localized_name": "冥魂大帝"
        },
        {
            "name": "npc_dota_hero_death_prophet",
            "id": 43,
            "localized_name": "死亡先知"
        },
        {
            "name": "npc_dota_hero_phantom_assassin",
            "id": 44,
            "localized_name": "幻影刺客"
        },
        {
            "name": "npc_dota_hero_pugna",
            "id": 45,
            "localized_name": "帕格纳"
        },
        {
            "name": "npc_dota_hero_templar_assassin",
            "id": 46,
            "localized_name": "圣堂刺客"
        },
        {
            "name": "npc_dota_hero_viper",
            "id": 47,
            "localized_name": "冥界亚龙"
        },
        {
            "name": "npc_dota_hero_luna",
            "id": 48,
            "localized_name": "露娜"
        },
        {
            "name": "npc_dota_hero_dragon_knight",
            "id": 49,
            "localized_name": "龙骑士"
        },
        {
            "name": "npc_dota_hero_dazzle",
            "id": 50,
            "localized_name": "戴泽"
        },
        {
            "name": "npc_dota_hero_rattletrap",
            "id": 51,
            "localized_name": "发条技师"
        },
        {
            "name": "npc_dota_hero_leshrac",
            "id": 52,
            "localized_name": "拉席克"
        },
        {
            "name": "npc_dota_hero_furion",
            "id": 53,
            "localized_name": "先知"
        },
        {
            "name": "npc_dota_hero_life_stealer",
            "id": 54,
            "localized_name": "噬魂鬼"
        },
        {
            "name": "npc_dota_hero_dark_seer",
            "id": 55,
            "localized_name": "黑暗贤者"
        },
        {
            "name": "npc_dota_hero_clinkz",
            "id": 56,
            "localized_name": "克林克兹"
        },
        {
            "name": "npc_dota_hero_omniknight",
            "id": 57,
            "localized_name": "全能骑士"
        },
        {
            "name": "npc_dota_hero_enchantress",
            "id": 58,
            "localized_name": "魅惑魔女"
        },
        {
            "name": "npc_dota_hero_huskar",
            "id": 59,
            "localized_name": "哈斯卡"
        },
        {
            "name": "npc_dota_hero_night_stalker",
            "id": 60,
            "localized_name": "暗夜魔王"
        },
        {
            "name": "npc_dota_hero_broodmother",
            "id": 61,
            "localized_name": "育母蜘蛛"
        },
        {
            "name": "npc_dota_hero_bounty_hunter",
            "id": 62,
            "localized_name": "赏金猎人"
        },
        {
            "name": "npc_dota_hero_weaver",
            "id": 63,
            "localized_name": "编织者"
        },
        {
            "name": "npc_dota_hero_jakiro",
            "id": 64,
            "localized_name": "杰奇洛"
        },
        {
            "name": "npc_dota_hero_batrider",
            "id": 65,
            "localized_name": "蝙蝠骑士"
        },
        {
            "name": "npc_dota_hero_chen",
            "id": 66,
            "localized_name": "陈"
        },
        {
            "name": "npc_dota_hero_spectre",
            "id": 67,
            "localized_name": "幽鬼"
        },
        {
            "name": "npc_dota_hero_doom_bringer",
            "id": 69,
            "localized_name": "末日使者"
        },
        {
            "name": "npc_dota_hero_ancient_apparition",
            "id": 68,
            "localized_name": "远古冰魄"
        },
        {
            "name": "npc_dota_hero_ursa",
            "id": 70,
            "localized_name": "熊战士"
        },
        {
            "name": "npc_dota_hero_spirit_breaker",
            "id": 71,
            "localized_name": "裂魂人"
        },
        {
            "name": "npc_dota_hero_gyrocopter",
            "id": 72,
            "localized_name": "矮人直升机"
        },
        {
            "name": "npc_dota_hero_alchemist",
            "id": 73,
            "localized_name": "炼金术士"
        },
        {
            "name": "npc_dota_hero_invoker",
            "id": 74,
            "localized_name": "祈求者"
        },
        {
            "name": "npc_dota_hero_silencer",
            "id": 75,
            "localized_name": "沉默术士"
        },
        {
            "name": "npc_dota_hero_obsidian_destroyer",
            "id": 76,
            "localized_name": "殁境神蚀者"
        },
        {
            "name": "npc_dota_hero_lycan",
            "id": 77,
            "localized_name": "狼人"
        },
        {
            "name": "npc_dota_hero_brewmaster",
            "id": 78,
            "localized_name": "酒仙"
        },
        {
            "name": "npc_dota_hero_shadow_demon",
            "id": 79,
            "localized_name": "暗影恶魔"
        },
        {
            "name": "npc_dota_hero_lone_druid",
            "id": 80,
            "localized_name": "德鲁伊"
        },
        {
            "name": "npc_dota_hero_chaos_knight",
            "id": 81,
            "localized_name": "混沌骑士"
        },
        {
            "name": "npc_dota_hero_meepo",
            "id": 82,
            "localized_name": "米波"
        },
        {
            "name": "npc_dota_hero_treant",
            "id": 83,
            "localized_name": "树精卫士"
        },
        {
            "name": "npc_dota_hero_ogre_magi",
            "id": 84,
            "localized_name": "食人魔魔法师"
        },
        {
            "name": "npc_dota_hero_undying",
            "id": 85,
            "localized_name": "不朽尸王"
        },
        {
            "name": "npc_dota_hero_rubick",
            "id": 86,
            "localized_name": "拉比克"
        },
        {
            "name": "npc_dota_hero_disruptor",
            "id": 87,
            "localized_name": "干扰者"
        },
        {
            "name": "npc_dota_hero_nyx_assassin",
            "id": 88,
            "localized_name": "司夜刺客"
        },
        {
            "name": "npc_dota_hero_naga_siren",
            "id": 89,
            "localized_name": "娜迦海妖"
        },
        {
            "name": "npc_dota_hero_keeper_of_the_light",
            "id": 90,
            "localized_name": "光之守卫"
        },
        {
            "name": "npc_dota_hero_wisp",
            "id": 91,
            "localized_name": "艾欧"
        },
        {
            "name": "npc_dota_hero_visage",
            "id": 92,
            "localized_name": "维萨吉"
        },
        {
            "name": "npc_dota_hero_slark",
            "id": 93,
            "localized_name": "斯拉克"
        },
        {
            "name": "npc_dota_hero_medusa",
            "id": 94,
            "localized_name": "美杜莎"
        },
        {
            "name": "npc_dota_hero_troll_warlord",
            "id": 95,
            "localized_name": "巨魔战将"
        },
        {
            "name": "npc_dota_hero_centaur",
            "id": 96,
            "localized_name": "半人马战行者"
        },
        {
            "name": "npc_dota_hero_magnataur",
            "id": 97,
            "localized_name": "马格纳斯"
        },
        {
            "name": "npc_dota_hero_shredder",
            "id": 98,
            "localized_name": "伐木机"
        },
        {
            "name": "npc_dota_hero_bristleback",
            "id": 99,
            "localized_name": "钢背兽"
        },
        {
            "name": "npc_dota_hero_tusk",
            "id": 100,
            "localized_name": "巨牙海民"
        },
        {
            "name": "npc_dota_hero_skywrath_mage",
            "id": 101,
            "localized_name": "天怒法师"
        },
        {
            "name": "npc_dota_hero_abaddon",
            "id": 102,
            "localized_name": "亚巴顿"
        },
        {
            "name": "npc_dota_hero_elder_titan",
            "id": 103,
            "localized_name": "上古巨神"
        },
        {
            "name": "npc_dota_hero_legion_commander",
            "id": 104,
            "localized_name": "军团指挥官"
        },
        {
            "name": "npc_dota_hero_ember_spirit",
            "id": 106,
            "localized_name": "灰烬之灵"
        },
        {
            "name": "npc_dota_hero_earth_spirit",
            "id": 107,
            "localized_name": "大地之灵"
        },
        {
            "name": "npc_dota_hero_terrorblade",
            "id": 109,
            "localized_name": "恐怖利刃"
        },
        {
            "name": "npc_dota_hero_phoenix",
            "id": 110,
            "localized_name": "凤凰"
        },
        {
            "name": "npc_dota_hero_oracle",
            "id": 111,
            "localized_name": "神谕者"
        },
        {
            "name": "npc_dota_hero_techies",
            "id": 105,
            "localized_name": "工程师"
        },
        {
            "name": "npc_dota_hero_winter_wyvern",
            "id": 112,
            "localized_name": "寒冬飞龙"
        },
        {
            "name": "npc_dota_hero_arc_warden",
            "id": 113,
            "localized_name": "天穹守望者"
        },
        {
            "name": "npc_dota_hero_abyssal_underlord",
            "id": 108,
            "localized_name": "孽主"
        },
        {
            "name": "npc_dota_hero_monkey_king",
            "id": 114,
            "localized_name": "齐天大圣"
        },
        {
            "name": "npc_dota_hero_pangolier",
            "id": 120,
            "localized_name": "石鳞剑士"
        },
        {
            "name": "npc_dota_hero_dark_willow",
            "id": 119,
            "localized_name": "邪影芳灵"
        },
        {
            "name": "npc_dota_hero_grimstroke",
            "id": 121,
            "localized_name": "天涯墨客"
        },
        {
            "name": "npc_dota_hero_mars",
            "id": 129,
            "localized_name": "玛尔斯"
        },
        {
            "name": "npc_dota_hero_void_spirit",
            "id": 126,
            "localized_name": "虚无之灵"
        },
        {
            "name": "npc_dota_hero_snapfire",
            "id": 128,
            "localized_name": "电炎绝手"
        },
        {
            "name": "npc_dota_hero_hoodwink",
            "id": 123,
            "localized_name": "森海飞霞"
        },
        {
            "name": "npc_dota_hero_dawnbreaker",
            "id": 135,
            "localized_name": "破晓辰星"
        },
        {
            "name": "npc_dota_hero_marci",
            "id": 136,
            "localized_name": "玛西"
        },
        {
            "name": "npc_dota_hero_primal_beast",
            "id": 137,
            "localized_name": "獸"
        }
    ]);
    let mut map = HashMap::new();
    for i in data.as_array().unwrap() {
        let hero_id = i["id"].as_i64();
        if let Some(hero_id) = hero_id {
            map.insert(hero_id, i["localized_name"].as_str().unwrap().to_string());
        }
    }
    Mutex::new(map)
});
