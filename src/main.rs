mod models;
use std::fs;
use crate::models::race::RaceList;

fn main() {
    let data = fs::read_to_string("data/races.toml").expect("Failed to read races file");
    let races: RaceList = toml::from_str(&data).expect("TOML parsing Error");

    for race in &races.race{
        println!("Race: {}", race.name);
        println!("  Description: {}", race.description);
        println!("  Speed: {}", race.speed);
        println!("  Bonuses: {:?}", race.ability_bonuses);
    }
}