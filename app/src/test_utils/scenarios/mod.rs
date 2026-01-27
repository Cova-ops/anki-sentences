pub mod gram_type;
pub mod niveau_liste;
pub mod worte;
pub mod worte_gender;
pub mod worte_gram_type;
pub mod worte_review;

#[derive(Clone)]
pub struct Scenario<T> {
    pub initial: Vec<T>,
    pub update: Vec<T>,
}
