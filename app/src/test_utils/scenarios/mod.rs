pub mod gram_type;
pub mod niveau_liste;
pub mod worte;
pub mod worte_gender;

#[derive(Clone)]
pub struct Scenario<T> {
    pub initial: Vec<T>,
    pub update: Vec<T>,
}
