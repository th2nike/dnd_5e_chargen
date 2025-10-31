use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat{
    pub name: Ability,
    pub value: u8,
}

impl fmt::Display for Stat{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        let name = self.name;
        let value = self.value;
        write!(f, "{:3}: {:2}", name, value)
    }
}

impl Stat{
    pub fn new(name: Ability, value: u8) -> Self{
        Self {name, value}
    }

    pub fn modifier(&self) -> i8{
        ((self.value as i8) - 10) / 2
    }

    pub fn increase(&mut self, amount: u8){
        self.value = self.value.saturating_add(amount)
    }

    pub fn decrease(&mut self, amount: u8){
        self.value = self.value.saturating_sub(amount)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl fmt::Display for Ability{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        let name = match self{
            Ability::Strength => "STR",
            Ability::Dexterity => "DEX",
            Ability::Constitution => "CON",
            Ability::Intelligence => "INT",
            Ability::Wisdom => "WIS",
            Ability::Charisma => "CHA,"
        };
        write!(f, "{}", name)
    }
}
