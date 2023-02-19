use std::collections::HashMap;

use prettytable::{
    format::{self, FormatBuilder, LinePosition, LineSeparator, TableFormat},
    row, Cell, Row, Table,
};
use uuid::Uuid;

struct WfrpCoreStat {
    name: String,
    base: u8,
    ext: u8,
    current: u8,
}

// impl WfrpCoreStats {
//     pub fn name_pl(self, stat: &str) -> &str {
//         match stat {
//             "ws" => "WW",
//             "bs" => "US",
//             "s" => "S",
//             "t" => "Wt",
//             "i" => "I",
//             "agi" => "Zw",
//             "dex" => "Zr",
//             "int" => "Int",
//             "wp" => "SW",
//             "fel" => "Ogd",
//             _ => "Nieznany",
//         }
//     }
// }

struct WfrpSkill {
    name: String,
    basic: bool,
    stat: String,
}

struct WfrpTalent {
    name: String,
    level: u8,
}

pub struct WfrpChar {
    id: String,
    name: String,
    player: String,
    stats: HashMap<String, WfrpCoreStat>,
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
    skills: HashMap<String, WfrpSkill>,
    talents: Vec<WfrpTalent>,
    wounds: u8,
}

impl WfrpChar {
    pub fn new(name: &str, player: &str) -> Self {
        let mut _stats = HashMap::<String, WfrpCoreStat>::new();
        let slist = ["WW", "US", "S", "Wt", "I", "Zw", "Zr", "Int", "SW", "Ogd"];
        for s in slist {
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
            skills: HashMap::new(),
            talents: Vec::new(),
            wounds: 0,
        }
    }

    pub fn print(&self) -> String {
        let mut lines: Vec<&str> = Vec::new();
        let mut l = format!("{} ({})", self.name, self.player);
        lines.push(&l);
        let mut table = Table::new();
        let tformat = FormatBuilder::new()
            .padding(1, 1)
            .column_separator('|')
            .build();
        table.set_format(tformat);
        let slist = ["WW", "US", "S", "Wt", "I", "Zw", "Zr", "Int", "SW", "Ogd"];
        table.add_row(row![
            "WW", "US", "S", "Wt", "I", "Zw", "Zr", "Int", "SW", "Ogd"
        ]);
        let r1 = slist.map(|s| Cell::new(&self.stats[s].base.to_string()));

        table.add_row(Row::new(r1.to_vec()));

        let binding = table.to_string();
        lines.push("```");
        lines.push(&binding);
        lines.push("```");
        lines.join("\n").to_string()
    }
}
