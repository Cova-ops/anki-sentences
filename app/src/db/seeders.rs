use color_eyre::eyre::Result;
use once_cell::sync::Lazy;
use rusqlite::Connection;

use crate::db::{
    gram_type::GramTypeRepo,
    niveau_liste::NiveauListeRepo,
    schemas::{
        gram_type::{GramTypeSchema, NewGramTypeSchema},
        niveau_liste::{NewNiveauListeSchema, NiveauListeSchema},
        worte_gender::{NewWorteGenderSchema, WorteGenderSchema},
    },
    worte_gender::WorteGenderRepo,
};

pub static SEED_WORTE_GENDER_LISTE: Lazy<Vec<NewWorteGenderSchema>> = Lazy::new(|| {
    Vec::from([
        NewWorteGenderSchema::new(0, "Maskuline", "der"),
        NewWorteGenderSchema::new(1, "Femenin", "die"),
        NewWorteGenderSchema::new(2, "Neutrum", "das"),
        NewWorteGenderSchema::new(3, "Plural", "die"),
    ])
});

pub static SEED_NIVEAU_LISTE: Lazy<Vec<NewNiveauListeSchema>> = Lazy::new(|| {
    Vec::from([
        NewNiveauListeSchema::new(0, "A1"),
        NewNiveauListeSchema::new(1, "A2"),
        NewNiveauListeSchema::new(2, "B1"),
        NewNiveauListeSchema::new(3, "B2"),
        NewNiveauListeSchema::new(4, "C1"),
        NewNiveauListeSchema::new(5, "C2"),
    ])
});

pub static SEED_GRAM_TYPE_LISTE: Lazy<Vec<NewGramTypeSchema>> = Lazy::new(|| {
    Vec::from([
        NewGramTypeSchema::new(0, "noun_common", "Sustantivo comun"),
        NewGramTypeSchema::new(1, "noun_proper", "Nombre propio"),
        NewGramTypeSchema::new(2, "verb_main", "Verbo lexico"),
        NewGramTypeSchema::new(3, "verb_modal", "Verbo modal"),
        NewGramTypeSchema::new(4, "verb_auxiliary", "Verbo auxiliar"),
        NewGramTypeSchema::new(5, "verb_separable", "Verbo separable"),
        NewGramTypeSchema::new(6, "verb_reflexive", "Verbo reflexivo"),
        NewGramTypeSchema::new(7, "adjective", "Adjetivo"),
        NewGramTypeSchema::new(8, "adverb_time", "Adverbio tiempo"),
        NewGramTypeSchema::new(9, "adverb_place", "Adverbio lugar"),
        NewGramTypeSchema::new(10, "adverb_manner", "Adverbio modo"),
        NewGramTypeSchema::new(11, "adverb_degree", "Adverbio grado"),
        NewGramTypeSchema::new(12, "adverb_sentence_connector", "Adverbio conector"),
        NewGramTypeSchema::new(13, "pronoun_personal", "Pronombre personal"),
        NewGramTypeSchema::new(14, "pronoun_possessive", "Pronombre posesivo"),
        NewGramTypeSchema::new(15, "pronoun_reflexive", "Pronombre reflexivo"),
        NewGramTypeSchema::new(16, "pronoun_demonstrative", "Pronombre demostrativo"),
        NewGramTypeSchema::new(17, "pronoun_relative", "Pronombre relativo"),
        NewGramTypeSchema::new(18, "pronoun_interrogative", "Pronombre interrogativo"),
        NewGramTypeSchema::new(19, "pronoun_indefinite", "Pronombre indefinido"),
        NewGramTypeSchema::new(20, "article_definite", "Articulo definido"),
        NewGramTypeSchema::new(21, "article_indefinite", "Articulo indefinido"),
        NewGramTypeSchema::new(22, "determiner_quantifier", "Determinante cuantificador"),
        NewGramTypeSchema::new(23, "preposition_dative", "Preposicion dativo"),
        NewGramTypeSchema::new(24, "preposition_akkusative", "Preposicion acusativo"),
        NewGramTypeSchema::new(25, "preposition_genitive", "Preposicion genitivo"),
        NewGramTypeSchema::new(26, "preposition_two_way", "Preposicion doble"),
        NewGramTypeSchema::new(27, "conjunction_coordinating", "Conjuncion coordinante"),
        NewGramTypeSchema::new(28, "conjunction_subordinating", "Conjuncion subordinante"),
        NewGramTypeSchema::new(29, "particle_modal", "Particula modal"),
        NewGramTypeSchema::new(30, "particle_focus", "Particula enfoque"),
        NewGramTypeSchema::new(31, "particle_negation", "Particula negacion"),
        NewGramTypeSchema::new(32, "particle_answer", "Particula respuesta"),
        NewGramTypeSchema::new(33, "numeral_cardinal", "Numeral cardinal"),
        NewGramTypeSchema::new(34, "numeral_ordinal", "Numeral ordinal"),
        NewGramTypeSchema::new(35, "interjection", "Interjeccion"),
        NewGramTypeSchema::new(36, "fixed_phrase", "Frase fija"),
        NewGramTypeSchema::new(37, "prefix_separable", "Prefijo separable"),
        NewGramTypeSchema::new(38, "pattern_verb_dativ", "Patron verbo dativo"),
        NewGramTypeSchema::new(39, "pattern_verb_akkusativ", "Patron verbo acusativo"),
        NewGramTypeSchema::new(40, "pattern_verb_dat_akk", "Patron verbo dativo-acusativo"),
    ])
});

pub fn init_data(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    // GenderWorte
    let data = WorteGenderRepo::bulk_insert_tx(&tx, &SEED_WORTE_GENDER_LISTE)?;
    WorteGenderSchema::init_data(&data)?;

    // NiveauWorte
    let data = NiveauListeRepo::bulk_insert_tx(&tx, &SEED_NIVEAU_LISTE)?;
    NiveauListeSchema::init_data(&data)?;

    // GramType
    let data = GramTypeRepo::bulk_insert_tx(&tx, &SEED_GRAM_TYPE_LISTE)?;
    GramTypeSchema::init_data(&data)?;

    tx.commit()?;

    Ok(())
}
