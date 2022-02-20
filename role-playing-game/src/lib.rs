// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            let mut new_mana = self.mana;
            if self.level >= 10 {
                new_mana = Some(100);
            }
            Some(Player {
                health: 100,
                mana: new_mana,
                level: self.level,
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        let mut mana_spent = 0;
        if self.mana.is_none() {
            self.health -= if self.health > mana_cost {
                mana_cost
            } else {
                self.health
            };
        } else if self.mana.unwrap() > 0 && mana_cost < self.mana.unwrap() {
            let total_damage = mana_cost * 2;
            self.mana = Some(self.mana.unwrap() - mana_cost);
            mana_spent = total_damage;
        }
        mana_spent
    }
}
