use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Race {
    pub name: String,
    pub description: String,
    pub speed: u8,
    pub size: String,
    pub languages: Vec<String>,
    pub ability_bonuses: HashMap<String, i8>,
    pub proficiencies: Vec<String>,
    pub traits: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RaceList{
    pub race: Vec<Race>,
}