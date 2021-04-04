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
            // mod 256 to ignore any false positives above the max score of 255
            score: score % 256,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut score = self.score;
        let mut allergens: Vec<Allergen> = Vec::new();

        let mut exp: u32 = 7;
        while score > 0 {
            let score_to_check = 2u32.pow(exp);
            if score >= score_to_check {
                let allergen = match score_to_check {
                    1 => Allergen::Eggs,
                    2 => Allergen::Peanuts,
                    4 => Allergen::Shellfish,
                    8 => Allergen::Strawberries,
                    16 => Allergen::Tomatoes,
                    32 => Allergen::Chocolate,
                    64 => Allergen::Pollen,
                    128 => Allergen::Cats,
                    _ => unreachable!("there are only 8 possible allergens"),
                };
                allergens.push(allergen);
                score -= score_to_check;
            }
            if exp == 0 {
                break;
            }
            exp -= 1;
        }

        allergens
    }
}
