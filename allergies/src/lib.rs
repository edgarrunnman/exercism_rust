pub struct Allergies {
    list: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs = 0b1,
    Peanuts = 0b10,
    Shellfish = 0b100,
    Strawberries = 0b1000,
    Tomatoes = 0b10000,
    Chocolate = 0b100000,
    Pollen = 0b1000000,
    Cats = 0b10000000,
}
use self::Allergen::*;
impl Allergies {
    pub fn new(score: u32) -> Self {
        let list = [
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ]
        .into_iter()
        .filter(|a| &score & a.clone() as u32 != 0)
        .collect::<Vec<Allergen>>();
        Allergies { list }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.list.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.list.clone()
    }
}
