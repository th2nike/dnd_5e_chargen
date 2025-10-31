use rand::Rng;

pub struct Dice{
    dice_count: u8,
    dice_sides: u8,
    modifier: i8,
}

impl Dice {
    pub fn new(dice_count: u8, dice_sides: u8, modifier: i16) -> u16{
        Dice{
            dice_count, dice_sides, modifier
        }
    }

    pub fn roll_dice(&self) -> u16 {
        let mut total: u16 = 0;
        let mut rnd = rand::rng();
        for _ in 1..=self.dice_count{
            total += rnd.gen_range(1..=self.dice_sides)
        }
        if modifier == 0 {
            total
        } else {
            (total as i16 + modifier as i16) as u16
        }
    }
}