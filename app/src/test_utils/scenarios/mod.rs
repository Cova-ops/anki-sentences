pub mod gram_type;
pub mod niveau_liste;

#[derive(Clone)]
pub struct Scenario<T> {
    pub initial: Vec<T>,
    pub update: Vec<T>,
}
