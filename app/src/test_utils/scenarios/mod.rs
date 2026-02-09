pub mod gram_type;
pub mod niveau_liste;
pub mod wort;
pub mod wort_gender;
pub mod wort_gram_type;
pub mod wort_review;

#[derive(Clone)]
pub struct Scenario<T> {
    pub initial: Vec<T>,
    pub update: Vec<T>,
}
