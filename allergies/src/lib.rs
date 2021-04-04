pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            score,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut score = self.score;
        let mut allergens: Vec<Allergen> = Vec::new(); 

        let mut exp: u32 = 0;
        while score > 0 && exp < 8 {
            if score % (2u32.pow(exp)) == 0 {
                let allergen = match exp {
                    0 => Allergen::Eggs,
                    1 => Allergen::Peanuts,
                    2 => Allergen::Shellfish,
                    3 => Allergen::Strawberries,
                    4 => Allergen::Tomatoes,
                    5 => Allergen::Chocolate,
                    6 => Allergen::Pollen,
                    7 => Allergen::Cats,
                    _ => unreachable!()
                };
                allergens.push(allergen);
                score -= 2u32.pow(exp);
            }
            exp += 1;
        }

        allergens
    }
}
