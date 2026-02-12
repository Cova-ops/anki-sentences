mod gram_type;
mod niveau_liste;
mod setze;
mod setze_audios;
mod setze_review;
mod wort;
mod wort_gender;
mod wort_gram_type;
mod wort_review;

pub use gram_type::*;
pub use niveau_liste::*;
pub use setze::*;
pub use setze_audios::*;
pub use setze_review::*;
pub use wort::*;
pub use wort_gender::*;
pub use wort_gram_type::*;
pub use wort_review::*;

#[derive(Clone)]
pub struct Scenario<T> {
    pub initial: Vec<T>,
    pub update: Vec<T>,
}
