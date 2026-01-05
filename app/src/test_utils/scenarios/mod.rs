pub mod gram_type;

#[derive(Clone)]
pub struct Scenario<T> {
    pub initial: Vec<T>,
    pub update: Vec<T>,
}
