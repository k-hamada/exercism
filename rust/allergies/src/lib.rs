#[macro_use]
extern crate bitflags;

bitflags! {
    pub struct Allergen: u32 {
        const Eggs = 0b00000001;
        const Peanuts = 0b00000010;
        const Shellfish = 0b00000100;
        const Strawberries = 0b00001000;
        const Tomatoes = 0b00010000;
        const Chocolate = 0b00100000;
        const Pollen = 0b01000000;
        const Cats = 0b10000000;
    }
}

impl Allergen {
    fn values() -> Vec<Allergen> {
        vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ]
    }
}

pub struct Allergies {
    score: Allergen,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            score: Allergen::from_bits_truncate(score),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score.contains(*allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::values()
            .into_iter()
            .filter(|allergen| self.is_allergic_to(&allergen))
            .collect::<Vec<_>>()
    }
}
