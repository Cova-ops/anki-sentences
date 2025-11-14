use once_cell::sync::Lazy;

use crate::db::schemas::schwirigkeit_liste::NewSchwirigkeitSchema;

pub static SEED_SCHWIRIGKEIT_LISTE: Lazy<Vec<NewSchwirigkeitSchema>> = Lazy::new(|| {
    Vec::from([
        NewSchwirigkeitSchema::new(0, "Einfag"),
        NewSchwirigkeitSchema::new(1, "Normal"),
        NewSchwirigkeitSchema::new(2, "Schwirig"),
    ])
});
