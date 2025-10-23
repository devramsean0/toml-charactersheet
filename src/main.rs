use std::fs;

use askama::Template;
use clap::Parser;

use crate::parser::CharacterSheet;
mod parser;

#[derive(Parser, Debug)]
#[clap(author = "Sean Outram", version, about)]
struct Args {
    #[arg(short, long)]
    path: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct SheetTemplate {
    sheet: CharacterSheet,
    calculated_score_modifier: Vec<CalculatedScoreModifier>,
    calculated_action: Vec<CalculatedAction>,
    calculated_skill_rows: Vec<Vec<CalculatedSkill>>,
    calculated_saving_throw: Vec<CalculatedSavingThrow>,
    formatted_equipment: Vec<FormattedEquipment>,
    formatted_feats: Vec<FormattedFeatures>,
    tooltips: Vec<Tooltip>,
}

fn main() {
    let args = Args::parse();
    let sheet = match parser::parse_sheet(args.path) {
        Ok(sheet) => sheet,
        Err(err) => {
            println!("Error reading sheet: {err}");
            return;
        }
    };

    let mut tooltips: Vec<Tooltip> = vec![];

    let mut calculated_score_modifier: Vec<CalculatedScoreModifier> = vec![];
    for score in sheet.clone().scores {
        let modifier = (score.value - 10.0) / 2.0;
        calculated_score_modifier.push(CalculatedScoreModifier {
            name: score.key,
            total: score.value,
            modifier: modifier.floor(),
        })
    }

    let mut calculated_action: Vec<CalculatedAction> = vec![];
    let mut calculated_action_counter = 1;
    for action in sheet.clone().actions {
        let bonus_block_obj = if let Some(bonus_block_name) = &action.bonus_block {
            calculated_score_modifier
                .iter()
                .find(|x| x.name == *bonus_block_name)
        } else {
            None
        };
        let mut bonus = bonus_block_obj.map(|obj| obj.modifier).unwrap_or(0.0);
        if action.proficient.unwrap_or(false) {
            println!("Added Profficiency Bonus to {}", action.name);
            bonus = bonus + sheet.metadata.clone().proficiency_bonus;
        }
        if action.magic_bonus.unwrap_or(false) {
            println!("Added Magic Bonus to {}", action.name);
            bonus = bonus + 1.0;
        }
        let mut text = String::new();
        if !action.text.is_empty() {
            text = format!("[a{calculated_action_counter}]");
            tooltips.push(Tooltip {
                key: text.clone(),
                text: markdown::to_html(action.text.as_str()),
            });
            calculated_action_counter = calculated_action_counter + 1;
        }
        let mut damage: Option<String> = action.damage.clone();
        let mut damage_bonus = bonus_block_obj
            .map(|obj: &CalculatedScoreModifier| obj.modifier)
            .unwrap_or(0.0);
        let mut rage_damage = action.damage;
        if damage.is_some() {
            if action.magic_bonus.unwrap_or(false) {
                println!("Added Magic Bonus to {} damage", action.name);
                damage_bonus = damage_bonus + 1.0;
            }
            damage = Some(format!("{}+{}", damage.clone().unwrap(), damage_bonus));
        }
        if rage_damage.is_some() && action.bonus_block == Some(String::from("str")) {
            let rage_bonus_f = sheet.metadata.clone().rage_bonus.unwrap_or(0) as f64;
            let total = damage_bonus + rage_bonus_f;
            rage_damage = Some(format!("{}+{}", rage_damage.clone().unwrap(), total));
            println!("Calculated Rage Damage for {}", action.name)
        }
        calculated_action.push(CalculatedAction {
            action_type: action.action_type,
            name: action.name,
            damage: damage,
            rage_damage: rage_damage,
            dmg_type: action.dmg_type,
            text: text,
            atk_bonus: bonus,
        });
    }

    let mut calculated_saving_throw: Vec<CalculatedSavingThrow> = vec![];
    for saving_throw in sheet.clone().saving_throws {
        let score_block_name = &saving_throw.key;
        let score_obj = calculated_score_modifier
            .iter()
            .find(|x| x.name == *score_block_name);
        let mut value = score_obj
            .map(|obj| obj)
            .unwrap_or(&CalculatedScoreModifier {
                name: String::new(),
                total: 0.0,
                modifier: 0.0,
            })
            .modifier;
        if saving_throw.proficient {
            value = value + sheet.clone().metadata.proficiency_bonus;
        };
        calculated_saving_throw.push(CalculatedSavingThrow {
            key: saving_throw.key,
            value: value,
            proficient: saving_throw.proficient,
        });
    }

    let mut calculated_skill: Vec<CalculatedSkill> = vec![];
    for skill in sheet.clone().skills {
        let score_block_name = &skill.score_key;
        let score_obj = calculated_score_modifier
            .iter()
            .find(|x| x.name == *score_block_name);
        let mut value = score_obj
            .map(|obj| obj)
            .unwrap_or(&CalculatedScoreModifier {
                name: String::new(),
                total: 0.0,
                modifier: 0.0,
            })
            .modifier;
        if skill.proficient {
            value = value + sheet.clone().metadata.proficiency_bonus;
        };
        calculated_skill.push(CalculatedSkill {
            key: skill.key,
            score_key: skill.score_key,
            value: value,
            proficient: skill.proficient,
        })
    }

    // Split skills into rows of 16 each
    let calculated_skill_rows: Vec<Vec<CalculatedSkill>> = calculated_skill
        .chunks(16)
        .map(|chunk| chunk.to_vec())
        .collect();

    let mut formatted_equipment: Vec<FormattedEquipment> = vec![];
    let mut formatted_equipment_counter = 1;
    for equipment in sheet.clone().equipment {
        let mut text = String::new();
        if !equipment.text.clone().unwrap_or(String::new()).is_empty() {
            text = format!("[e{formatted_equipment_counter}]");
            tooltips.push(Tooltip {
                key: text.clone(),
                text: markdown::to_html(equipment.text.unwrap_or(String::new()).as_str()),
            });
            formatted_equipment_counter += 1;
        }
        formatted_equipment.push(FormattedEquipment {
            amount: equipment.amount,
            name: equipment.name,
            text: text,
        });
    }

    let mut formatted_features: Vec<FormattedFeatures> = vec![];
    for feat in sheet.clone().features {
        let mkdn = markdown::to_html(feat.text.as_str());
        formatted_features.push(FormattedFeatures {
            title: feat.title,
            text: mkdn,
        });
    }

    let html = SheetTemplate {
        sheet: sheet,
        calculated_score_modifier: calculated_score_modifier,
        calculated_action: calculated_action,
        calculated_skill_rows: calculated_skill_rows,
        calculated_saving_throw: calculated_saving_throw,
        formatted_equipment: formatted_equipment,
        formatted_feats: formatted_features,
        tooltips: tooltips,
    }
    .render()
    .expect("template should be valid");
    fs::write("./sheet.html", html).unwrap();
}

#[derive(Debug, Default)]
struct CalculatedScoreModifier {
    name: String,
    total: f64,
    modifier: f64,
}

#[derive(Debug)]
struct CalculatedAction {
    action_type: String,
    name: String,
    damage: Option<String>,
    rage_damage: Option<String>,
    atk_bonus: f64,
    dmg_type: Option<String>,
    text: String,
}

#[derive(Debug, Clone)]
struct CalculatedSkill {
    key: String,
    score_key: String,
    value: f64,
    proficient: bool,
}

#[derive(Debug)]
struct CalculatedSavingThrow {
    key: String,
    value: f64,
    proficient: bool,
}

#[derive(Debug)]
struct FormattedEquipment {
    amount: Option<i64>,
    name: String,
    text: String,
}

#[derive(Debug)]
struct FormattedFeatures {
    pub title: String,
    pub text: String,
}

struct Tooltip {
    pub key: String,
    pub text: String,
}
