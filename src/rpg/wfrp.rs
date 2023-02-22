use std::{collections::HashMap, str::Utf8Error};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

static STAT_LIST_PL: Lazy<Vec<&str>> =
    Lazy::new(|| ["WW", "US", "S", "Wt", "Zw", "I", "Zr", "Int", "SW", "Ogd"].to_vec());

pub fn wfrp_find_stat_pl(stat: &str) -> String {
    for s in STAT_LIST_PL.iter() {
        if s.to_lowercase() == stat.to_lowercase() {
            return s.to_string();
        }
    }
    return "".to_string();
}

static BASIC_SKILLS_PL: Lazy<HashMap<&str, &str>> = Lazy::new(|| wfrp_init_dicts_pl().unwrap());

pub fn wfrp_init_dicts_pl() -> Result<HashMap<&'static str, &'static str>, Utf8Error> {
    let mut basic_skills_pl = HashMap::<&'static str, &'static str>::new();
    basic_skills_pl.insert("Atletyka", "Zw");
    basic_skills_pl.insert("Broń Biała (Podstawowa)", "WW");
    basic_skills_pl.insert("Charyzma", "Ogd");
    basic_skills_pl.insert("Dowodzenie", "Ogd");
    basic_skills_pl.insert("Hazard", "Int");
    basic_skills_pl.insert("Intuicja", "I");
    basic_skills_pl.insert("Jeździectwo", "Zw");
    basic_skills_pl.insert("Mocna głowa", "Wt");
    basic_skills_pl.insert("Nawigacja", "I");
    basic_skills_pl.insert("Odporność", "Wt");
    basic_skills_pl.insert("Opanowanie", "SW");
    basic_skills_pl.insert("Oswajanie", "SW");
    basic_skills_pl.insert("Percepcja", "I");
    basic_skills_pl.insert("Plotkowanie", "Ogd");
    basic_skills_pl.insert("Powożenie", "Zw");
    basic_skills_pl.insert("Przekupstwo", "Ogd");
    basic_skills_pl.insert("Skradanie", "Zw");
    basic_skills_pl.insert("Sztuka", "Zr");
    basic_skills_pl.insert("Sztuka Przetrwania", "Int");
    basic_skills_pl.insert("Targowanie", "Ogd");
    basic_skills_pl.insert("Unik", "Zw");
    basic_skills_pl.insert("Wioślarstwo", "S");
    basic_skills_pl.insert("Wspinaczka", "S");
    basic_skills_pl.insert("Występy", "Ogd");
    basic_skills_pl.insert("Zastraszanie", "S");
    Ok(basic_skills_pl)
}

#[derive(Serialize, Deserialize)]
pub struct WfrpCoreStat {
    name: String,
    pub base: u8,
    pub ext: u8,
    pub current: u8,
}

#[derive(Serialize, Deserialize)]
pub struct WfrpSkill {
    name: String,
    level: u8,
    basic: bool,
    stat: String,
}

#[derive(Serialize, Deserialize)]
pub struct WfrpTalent {
    name: String,
    level: u8,
}

#[derive(Serialize, Deserialize)]
pub struct WfrpChar {
    pub id: String,
    name: String,
    player: String,
    pub stats: HashMap<String, WfrpCoreStat>,
    race: String,
    class: String,
    profession: String,
    profession_level: u8,
    hero_pts: u8,
    determination_pts: u8,
    motivation: String,
    destiny_pts: u8,
    luck_pts: u8,
    xp_actual: u16,
    xp_spent: u16,
    xp_total: u16,
    pub skills: HashMap<String, WfrpSkill>,
    pub talents: Vec<WfrpTalent>,
    wounds: u8,
}

impl WfrpChar {
    pub fn new(name: &str, player: &str) -> Self {
        let mut _stats = HashMap::<String, WfrpCoreStat>::new();
        for s in STAT_LIST_PL.iter() {
            _stats.insert(
                s.to_string(),
                WfrpCoreStat {
                    name: s.to_string(),
                    base: 20,
                    ext: 0,
                    current: 20,
                },
            );
        }
        let mut _skills = HashMap::<String, WfrpSkill>::new();
        for (k, v) in BASIC_SKILLS_PL.iter() {
            _skills.insert(
                k.to_string(),
                WfrpSkill {
                    name: k.to_string(),
                    level: 0,
                    basic: true,
                    stat: v.to_string(),
                },
            );
        }
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            player: player.to_string(),
            stats: _stats,
            race: "".to_string(),
            class: "".to_string(),
            profession: "".to_string(),
            profession_level: 0,
            hero_pts: 0,
            determination_pts: 0,
            motivation: "".to_string(),
            destiny_pts: 0,
            luck_pts: 0,
            xp_actual: 0,
            xp_spent: 0,
            xp_total: 0,
            skills: _skills,
            talents: Vec::new(),
            wounds: 0,
        }
    }

    pub fn print(&self, template: &str) -> String {
        let h = handlebars::Handlebars::new();
        let res = h.render_template(template, self);
        match res {
            Ok(response) => response,
            Err(_) => "".to_string(),
        }
    }
}
