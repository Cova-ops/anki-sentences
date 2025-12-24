use std::collections::{HashMap, HashSet};

use color_eyre::eyre::Result;
use rusqlite::Connection;

use crate::{
    db::{setze::SetzeRepo, worte::WorteRepo},
    helpers::{
        audios::{ManageAudios, audio_player::AudioPlayer},
        ui,
    },
    utils,
};

#[derive(Debug)]
struct ManageRepetitions {
    once_mistake: bool,
    repetition: u8,
}

impl ManageRepetitions {
    fn new_error() -> Self {
        Self {
            once_mistake: true,
            repetition: 0,
        }
    }

    fn new() -> Self {
        Self {
            once_mistake: false,
            repetition: 1,
        }
    }

    fn add_repetition(&mut self) {
        self.repetition += 1;
    }
}

const TEXT_SETZE_ONCE: &str = r##"
Para salir pon la palara "exit".                 Faltantes: {remainding}
Algunas letras que te pueden ayudar. :)
          - ß ẞ ä ö ü Ä Ö Ü 

  Tema: {thema}
  Oración: {satz}

Por favor traducela...
"##;

/// return:
/// - Seguir o no seguir con el proceso:
///   - 0 Finishing sentences
///   - 1 User typed "exit"
/// - Vec<(i32, u8)>:
///   - 1° -> satz_id
///   - 2° -> resultado usuario
///     - 0 -> No se la sabe
///     - 1 -> Se la sabe pero dificil
///     - 2 -> Se la sabe facil
pub fn make_setze_exercise_repeat(
    conn: &Connection,
    ids_setze: Vec<i32>,
    hash_audios: HashSet<i32>,
    manage_audio: &ManageAudios,
    batch: usize,
) -> Result<(i32, Vec<(i32, u8)>)> {
    let mut ids_setze = ids_setze;

    let mut vec_out: Vec<(i32, u8)> = vec![];
    let mut val_out = 0;
    let mut already_studied: HashMap<i32, ManageRepetitions> = HashMap::new();

    let take = ids_setze.len().min(batch);
    let aux_ids: Vec<i32> = ids_setze.drain(..take).collect();

    // Obtenemos toda la info del bloque de palabras que vamos a usar
    let mut setze_correct = SetzeRepo::fetch_by_id(conn, &aux_ids)?;

    let player = AudioPlayer::new();
    while !setze_correct.is_empty() {
        let s = setze_correct[0].clone();

        utils::console::clean_screen();
        let setze_remaining = setze_correct.len() + ids_setze.len();
        println!(
            "{}",
            TEXT_SETZE_ONCE
                .replace("{satz}", &s.setze_spanisch)
                .replace("{thema}", &s.thema)
                .replace("{remainding}", &setze_remaining.to_string())
        );

        #[allow(clippy::collapsible_if)]
        if let Some(audio) = hash_audios.get(&s.id) {
            if let Ok(Some(path)) = manage_audio.get_audio_setze(*audio) {
                player.play(path)?;
            }
        };

        let Some(input) = ui::prompt_nonempty("> ")? else {
            continue;
        };

        if input == "exit" {
            val_out = 1;
            break;
        }

        let correct_answer = utils::string::clean_sentences(&s.setze_deutsch);
        let input = utils::string::clean_sentences(&input);
        if input == correct_answer {
            if let Some(rep) = already_studied.get_mut(&s.id) {
                if rep.repetition < 1 {
                    // Primera vez que la acierta: subimos contador pero aún no la graduamos
                    rep.add_repetition();
                    setze_correct.rotate_left(1); // mueve el primer elemento al final del vector
                } else {
                    // Si la bandera de once_mistake esta en true, quiere decir que se equivoco con la
                    // palabra por lo menos una vez
                    let easy = if rep.once_mistake { 1 } else { 2 };
                    vec_out.push((s.id, easy));
                    setze_correct.remove(0);

                    if !ids_setze.is_empty() {
                        // Consultamos una nueva palabra y la añadimos al arreglo para su estudio
                        let id_new = ids_setze.remove(0);
                        let satz_new = SetzeRepo::fetch_by_id(conn, &[id_new])?;
                        setze_correct.push(satz_new[0].clone());
                    }

                    // limpiamos el hashmap de la palabra que ya no se va a repetir
                    already_studied.remove(&s.id);
                }
            } else {
                // La tuvo correcta a la primera
                let easy = 2;
                vec_out.push((s.id, easy));
                setze_correct.remove(0);

                if !ids_setze.is_empty() {
                    // Consultamos una nueva palabra y la añadimos al arreglo para su estudio
                    let id_new = ids_setze.remove(0);
                    let satz_new = SetzeRepo::fetch_by_id(conn, &[id_new])?;
                    setze_correct.push(satz_new[0].clone());
                }
            }

            continue;
        }

        already_studied
            .entry(s.id)
            .and_modify(|r| *r = ManageRepetitions::new_error())
            .or_insert(ManageRepetitions::new_error());

        println!();
        println!("Palabra incorrecta");
        println!("La palabra correcta es: {}", correct_answer);
        println!();

        loop {
            let Some(input) = ui::prompt_nonempty("> ")? else {
                break;
            };
            if input == "exit" {
                val_out = 1;
                break;
            }

            let input = input.trim();
            if input == correct_answer {
                setze_correct.rotate_left(1); // mueve el primer elemento al final del vector
                break;
            }
        }
    }

    Ok((val_out, vec_out))
}

const TEXT_WORTE_ONCE: &str = r##"
Para salir pon la palara "exit".                 Faltantes: {remainding}
Algunas letras que te pueden ayudar. :)
          - ß ẞ ä ö ü Ä Ö Ü 

  Tipo: {gram_type}
  Palabra: {wort}

Consideraciones:
  - Se esta contemplando el uso de Mayusculas y minusculas.
  - Para los sustantivos, favor de poner el articulo y el sustantivo. Ejem: "der Hund"

Por favor traducela...
"##;

/// return:
/// - Seguir o no seguir con el proceso:
///   - 0 Finishing sentences
///   - 1 User typed "exit"
/// - Vec<(i32, u8)>:
///   - 1° -> wort_id
///   - 2° -> resultado usuario
///     - 0 -> No se la sabe
///     - 1 -> Se la sabe pero dificil
///     - 2 -> Se la sabe facil
pub fn make_worte_exercise_repeat(
    conn: &Connection,
    ids_worte: Vec<i32>,
    hash_audios: HashSet<i32>,
    manage_audio: &ManageAudios,
    batch: usize,
) -> Result<(i32, Vec<(i32, u8)>)> {
    let mut ids_worte = ids_worte;

    let mut vec_out: Vec<(i32, u8)> = vec![];
    let mut val_out = 0;
    let mut already_studied: HashMap<i32, ManageRepetitions> = HashMap::new();

    let take = ids_worte.len().min(batch);
    let aux_ids: Vec<i32> = ids_worte.drain(..take).collect();

    // Obtenemos toda la info del bloque de palabras que vamos a usar
    let mut worte_correct = WorteRepo::fetch_by_id(conn, &aux_ids)?;

    let player = AudioPlayer::new();
    while !worte_correct.is_empty() && val_out == 0 {
        let w = worte_correct[0].clone();

        utils::console::clean_screen();
        let worte_remaining = worte_correct.len() + ids_worte.len();
        println!(
            "{}",
            TEXT_WORTE_ONCE
                .replace("{wort}", &w.worte_es)
                .replace("{remainding}", &worte_remaining.to_string())
                .replace(
                    "{gram_type}",
                    &w.gram_type_id
                        .into_iter()
                        .map(|r| format!("{} ", r.name))
                        .collect::<Vec<_>>()
                        .join(",")
                )
        );

        #[allow(clippy::collapsible_if)]
        if let Some(audio) = hash_audios.get(&w.id) {
            if let Ok(Some(path)) = manage_audio.get_audio_worte(*audio) {
                player.play(path)?;
            }
        };

        let Some(input) = ui::prompt_nonempty("> ")? else {
            continue;
        };

        if input == "exit" {
            val_out = 1;
            break;
        }

        let correct_answer = match w.gender_id {
            Some(v) => format!("{} {}", v.artikel.to_lowercase(), w.worte_de),
            None => w.worte_de.clone(),
        };

        let input = input.trim();
        if input == correct_answer {
            if let Some(rep) = already_studied.get_mut(&w.id) {
                if rep.repetition < 1 {
                    // Primera vez que la acierta: subimos contador pero aún no la graduamos
                    rep.add_repetition();
                    worte_correct.rotate_left(1); // mueve el primer elemento al final del vector
                } else {
                    // Si la bandera de once_mistake esta en true, quiere decir que se equivoco con la
                    // palabra por lo menos una vez
                    let easy = if rep.once_mistake { 1 } else { 2 };
                    vec_out.push((w.id, easy));
                    worte_correct.remove(0);

                    if !ids_worte.is_empty() {
                        // Consultamos una nueva palabra y la añadimos al arreglo para su estudio
                        let id_new = ids_worte.remove(0);
                        let wort_new = WorteRepo::fetch_by_id(conn, &[id_new])?;
                        worte_correct.push(wort_new[0].clone());
                    }

                    // limpiamos el hashmap de la palabra que ya no se va a repetir
                    already_studied.remove(&w.id);
                }
            } else {
                // La tuvo correcta a la primera
                let easy = 2;
                vec_out.push((w.id, easy));
                worte_correct.remove(0);

                if !ids_worte.is_empty() {
                    // Consultamos una nueva palabra y la añadimos al arreglo para su estudio
                    let id_new = ids_worte.remove(0);
                    let wort_new = WorteRepo::fetch_by_id(conn, &[id_new])?;
                    worte_correct.push(wort_new[0].clone());
                }
            }

            continue;
        }

        already_studied
            .entry(w.id)
            .and_modify(|r| *r = ManageRepetitions::new_error())
            .or_insert(ManageRepetitions::new_error());

        println!();
        println!("Palabra incorrecta");
        println!("La palabra correcta es: {}", correct_answer);

        println!();
        println!("Ejemplo: {}", w.example_de);
        println!("Traducción: {}", w.example_es);
        println!();

        loop {
            let Some(input) = ui::prompt_nonempty("> ")? else {
                break;
            };
            if input == "exit" {
                val_out = 1;
                break;
            }

            let input = input.trim();
            if input == correct_answer {
                worte_correct.rotate_left(1); // mueve el primer elemento al final del vector
                break;
            }
        }
    }

    Ok((val_out, vec_out))
}

// TODO: Change storage of audio to home dir
