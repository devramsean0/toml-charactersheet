use serde::Deserialize;
use std::fs::read_to_string;

pub fn parse_sheet(path: String) -> Result<CharacterSheet, anyhow::Error> {
    let raw = read_to_string(path)?;
    let raw_str = raw.as_str();
    let sheet: CharacterSheet = toml::from_str(raw_str)?;

    Ok(sheet)
}

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterSheet {
    pub metadata: CharacterMetadata,
    pub scores: Vec<CharacterScores>,
    pub speeds: CharacterSpeeds,
    pub health: CharacterHealth,
    pub actions: Vec<CharacterAction>,
    pub equipment: Vec<CharacterEquipment>,
    pub money: CharacterMoney,
    pub features: Vec<CharacterFeatures>,
    pub saving_throws: Vec<CharacterSavingThrows>,
    pub skills: Vec<CharacterSkills>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterMetadata {
    pub name: String,
    pub class: String,
    pub level: i64,
    pub race: String,
    pub alignment: String,
    pub background: String,
    pub player_name: String,
    pub proficiency_bonus: f64,
    pub initiative: f64,
    pub passive_perception: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterScores {
    pub key: String,
    pub value: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterSpeeds {
    pub walking: i64,
    pub swimming: Option<i64>,
    pub climbing: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterHealth {
    pub max: i64,
    pub current: i64,
    pub temp: i64,
    pub dice: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterAction {
    pub action_type: String,
    pub name: String,
    pub bonus_block: Option<String>,
    pub profficient: Option<bool>,
    pub damage: Option<String>,
    pub dmg_type: Option<String>,
    pub magic_bonus: Option<bool>,
    pub text: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterEquipment {
    pub name: String,
    pub amount: Option<i64>,
    pub text: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterMoney {
    pub cp: Option<i64>,
    pub sp: Option<i64>,
    pub ep: Option<i64>,
    pub gp: Option<i64>,
    pub pp: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterFeatures {
    pub title: String,
    pub text: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterSavingThrows {
    pub key: String,
    pub profficient: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterSkills {
    pub key: String,
    pub score_key: String,
    pub profficient: bool,
}
