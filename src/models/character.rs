use serde::Deserialize;

use crate::models::{class::Class, race::Race, stat::Stat};

pub struct Character{
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub stats: Vec<Stat>,
    pub level: u8,
    pub hp: u16,
}

// impl Character{
//     pub fn new(name: &str, race: Race, class: Class) -> Self{

//     }
// }