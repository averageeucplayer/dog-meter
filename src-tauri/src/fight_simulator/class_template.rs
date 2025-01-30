use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct ClassTemplate<'a> {
    pub id: u32,
    pub name: &'a str,
    pub awakening_skill_id: u32,
    pub hyper_awakening_technique_skill_id: u32,
    pub hyper_awakening_skill_id: u32,
    pub identity_skill_ids: Vec<u32>,
    pub skill_ids: Vec<u32>
}

pub static DPS_CLASS_TEMPLATES: Lazy<Vec<ClassTemplate>> = Lazy::new(|| {

    vec![
        ClassTemplate {
            id: 102,
            name: "Berserker",
            awakening_skill_id: 16710,
            hyper_awakening_skill_id: 16730,
            hyper_awakening_technique_skill_id: 16650,
            identity_skill_ids: vec![16141],
            skill_ids: vec![16120, 16640, 16300, 16080, 16600, 16220, 16630, 16600]
        },
        ClassTemplate {
            id: 103,
            name: "Destroyer",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 104,
            name: "Gunlancer",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 112,
            name: "Slayer",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 202,
            name: "Arcanist",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 203,
            name: "Summoner",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 205,
            name: "Sorceress",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 302,
            name: "Wardancer",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 303,
            name: "Scrapper",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 304,
            name: "Soulfist",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 305,
            name: "Glaivier",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 312,
            name: "Striker",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 313,
            name: "Breaker",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 402,
            name: "Deathblade",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 403,
            name: "Shadowhunter",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 404,
            name: "Reaper",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 405,
            name: "Souleater",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 502,
            name: "Sharpshooter",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 503,
            name: "Deadeye",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 504,
            name: "Artillerist",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 505,
            name: "Machinist",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 512,
            name: "Gunslinger",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 601,
            name: "Specialist",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 603,
            name: "Aeromancer",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
    ]
});

pub static SUP_CLASS_TEMPLATES: Lazy<Vec<ClassTemplate>> = Lazy::new(|| {

    vec![
        ClassTemplate {
            id: 204,
            name: "Bard",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 105,
            name: "Paladin",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        },
        ClassTemplate {
            id: 602,
            name: "Artist",
            awakening_skill_id: 0,
            hyper_awakening_skill_id: 0,
            hyper_awakening_technique_skill_id: 0,
            identity_skill_ids: vec![0],
            skill_ids: vec![1, 2, 3, 4, 5, 6, 7, 8]
        }
    ]
});