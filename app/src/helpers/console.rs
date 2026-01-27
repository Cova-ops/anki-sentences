use std::collections::{HashMap, HashSet};

use color_eyre::eyre::Result;
use rand::seq::SliceRandom;
use rusqlite::Connection;

use crate::{
    db::{schemas::worte_gender::GenderGermanListe, setze::SetzeRepo, worte::WorteRepo},
    helpers::{
        audios::{ManageAudios, audio_player::AudioPlayer},
        ui,
    },
    services::tts::eleven_labs::LanguageVoice,
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
            if let Ok(Some(path)) = manage_audio.get_audio_setze(*audio, LanguageVoice::Spanisch) {
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

const TEXT_WORTE_WRITE: &str = r##"
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
pub fn make_worte_exercise_write(
    conn: &Connection,
    ids_worte: Vec<i32>,
    hash_audios: HashSet<i32>,
    manage_audio: &ManageAudios,
    batch: usize,
    no_shuffle: bool,
    lang: LanguageVoice,
) -> Result<(i32, Vec<(i32, u8)>)> {
    let mut ids_worte = ids_worte;

    let mut vec_out: Vec<(i32, u8)> = vec![];
    let mut val_out = 0;
    let mut already_studied: HashMap<i32, ManageRepetitions> = HashMap::new();

    let lang_second_audio = match lang {
        LanguageVoice::Spanisch => LanguageVoice::Deutsch,
        LanguageVoice::Deutsch => LanguageVoice::Spanisch,
    };

    let take = ids_worte.len().min(batch);
    let aux_ids: Vec<i32> = ids_worte.drain(..take).collect();

    // Obtenemos toda la info del bloque de palabras que vamos a usar
    let mut worte_correct = WorteRepo::fetch_by_id(conn, &aux_ids)?;
    if no_shuffle {
        let mut rng = rand::rng();
        worte_correct.shuffle(&mut rng);
    }

    let player = AudioPlayer::new();
    while !worte_correct.is_empty() && val_out == 0 {
        let w = worte_correct[0].clone();

        utils::console::clean_screen();
        let worte_remaining = worte_correct.len() + ids_worte.len();
        let worte = match lang {
            LanguageVoice::Spanisch => &w.worte_es,
            LanguageVoice::Deutsch => match w.gender_id.as_ref() {
                Some(v) => &format!("{} {}", v.artikel.to_lowercase(), w.worte_de),
                None => &w.worte_de,
            },
        };
        println!(
            "{}",
            TEXT_WORTE_WRITE
                .replace("{wort}", worte)
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
            if lang == LanguageVoice::Deutsch {
                if let Some(gender) = w.gender_id.as_ref() {
                    let path_artikel = manage_audio
                        .get_audio_artikel(GenderGermanListe::try_from(gender.artikel.as_str())?);

                    if let Ok(path) = path_artikel {
                        player.play(path)?;
                    }
                }
            }

            let path_word = manage_audio.get_audio_worte(*audio, lang);
            if let Ok(Some(path)) = path_word {
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

        let correct_answer: Vec<String> = if lang == LanguageVoice::Spanisch {
            let out = match w.gender_id.as_ref() {
                Some(v) => format!("{} {}", v.artikel.to_lowercase(), w.worte_de),
                None => w.worte_de.clone(),
            };
            vec![out]
        } else {
            let mut depth = 0usize;

            let cleaned: String = w
                .worte_es
                .chars()
                .filter_map(|c| match c {
                    '(' => {
                        depth += 1;
                        None
                    }
                    ')' => {
                        depth = depth.saturating_sub(1);
                        None
                    }
                    _ if depth > 0 => None,
                    _ => Some(c),
                })
                .collect();

            cleaned
                .split('/')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .collect()
        };

        let input = input.trim().to_owned();
        if correct_answer.contains(&input) {
            if let Some(rep) = already_studied.get_mut(&w.id) {
                if rep.repetition < 1 {
                    // Primera vez que la acierta: subimos contador pero aún no la guardamos
                    rep.add_repetition();
                    worte_correct.rotate_left(1); // mueve el primer elemento al final del vector
                } else {
                    // Si la bandera de once_mistake esta en true, quiere decir que se equivoco con la
                    // palabra por lo menos una vez
                    let easy = 1;
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
        println!("La palabra correcta es: {}", correct_answer.join(" / "));

        println!();
        println!("Ejemplo: {}", w.example_de);
        println!("Traducción: {}", w.example_es);
        println!();

        #[allow(clippy::collapsible_if)]
        if let Some(audio) = hash_audios.get(&w.id) {
            if lang_second_audio == LanguageVoice::Deutsch {
                if let Some(gender) = w.gender_id.as_ref() {
                    let path_artikel = manage_audio
                        .get_audio_artikel(GenderGermanListe::try_from(gender.artikel.as_str())?)?;

                    player.play(path_artikel)?;
                }
            }

            let path_word = manage_audio.get_audio_worte(*audio, lang_second_audio);
            if let Ok(Some(path)) = path_word {
                player.play(path)?;
            }
        };

        loop {
            let Some(input) = ui::prompt_nonempty("> ")? else {
                break;
            };
            if input == "exit" {
                val_out = 1;
                break;
            }

            let input = input.trim().to_owned();
            if correct_answer.contains(&input) {
                worte_correct.rotate_left(1); // mueve el primer elemento al final del vector
                break;
            }
        }
    }

    Ok((val_out, vec_out))
}

const TEXT_WORTE_SPEAK: &str = r##"
Words remaining: {remaining}

Category : {gram_type}
Word     : {wort}

What do you want to do?
  [r] 🔊 Repeat audio
  [1] ✅ I know this word
  [0] ❌ I don’t know this word
  [q] 🚪 Exit review

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
pub fn make_worte_exercise_speak(
    conn: &Connection,
    ids_worte: Vec<i32>,
    hash_audios: HashSet<i32>,
    manage_audio: &ManageAudios,
    batch: usize,
    no_shuffle: bool,
    lang: LanguageVoice,
) -> Result<(i32, Vec<(i32, u8)>)> {
    let mut ids_worte = ids_worte;

    let mut vec_out: Vec<(i32, u8)> = vec![];
    let mut val_out = 0;
    let mut already_studied: HashMap<i32, ManageRepetitions> = HashMap::new();

    let lang_second_audio = match lang {
        LanguageVoice::Spanisch => LanguageVoice::Deutsch,
        LanguageVoice::Deutsch => LanguageVoice::Spanisch,
    };

    let take = ids_worte.len().min(batch);
    let aux_ids: Vec<i32> = ids_worte.drain(..take).collect();

    // Obtenemos toda la info del bloque de palabras que vamos a usar
    let mut worte_correct = WorteRepo::fetch_by_id(conn, &aux_ids)?;
    if no_shuffle {
        let mut rng = rand::rng();
        worte_correct.shuffle(&mut rng);
    }

    let player = AudioPlayer::new();
    while !worte_correct.is_empty() && val_out == 0 {
        let w = worte_correct[0].clone();

        utils::console::clean_screen();
        let worte_remaining = worte_correct.len() + ids_worte.len();
        let worte = match lang {
            LanguageVoice::Spanisch => &w.worte_es,
            LanguageVoice::Deutsch => match w.gender_id.as_ref() {
                Some(v) => &format!("{} {}", v.artikel.to_lowercase(), w.worte_de),
                None => &w.worte_de,
            },
        };
        println!(
            "{}",
            TEXT_WORTE_SPEAK
                .replace("{wort}", worte)
                .replace("{remaining}", &worte_remaining.to_string())
                .replace(
                    "{gram_type}",
                    &w.gram_type_id
                        .into_iter()
                        .map(|r| format!("{} ", r.name))
                        .collect::<Vec<_>>()
                        .join(",")
                )
        );

        player.play_from_path(manage_audio, hash_audios.get(&w.id), &w.gender_id, lang)?;

        let input: String = loop {
            let Some(input) = ui::prompt_nonempty("> ")? else {
                continue;
            };

            match input.as_ref() {
                "1" | "q" | "0" | "exit" => break input,
                "r" => {
                    player.play_from_path(
                        manage_audio,
                        hash_audios.get(&w.id),
                        &w.gender_id,
                        lang,
                    )?;
                }
                _ => {}
            }
        };

        if input == "q" || input == "exit" {
            val_out = 1;
            break;
        }

        let correct_answer: Vec<String> = if lang == LanguageVoice::Spanisch {
            let out = match w.gender_id.as_ref() {
                Some(v) => format!("{} {}", v.artikel.to_lowercase(), w.worte_de),
                None => w.worte_de.clone(),
            };
            vec![out]
        } else {
            let mut depth = 0usize;

            let cleaned: String = w
                .worte_es
                .chars()
                .filter_map(|c| match c {
                    '(' => {
                        depth += 1;
                        None
                    }
                    ')' => {
                        depth = depth.saturating_sub(1);
                        None
                    }
                    _ if depth > 0 => None,
                    _ => Some(c),
                })
                .collect();

            cleaned
                .split('/')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .collect()
        };

        let input = input.trim().to_owned();
        if input == "1" {
            // Correct answer
            if let Some(rep) = already_studied.get_mut(&w.id) {
                if rep.repetition < 1 {
                    // Primera vez que la acierta: subimos contador pero aún no la guardamos
                    rep.add_repetition();
                    worte_correct.rotate_left(1); // mueve el primer elemento al final del vector
                } else {
                    // Si la bandera de once_mistake esta en true, quiere decir que se equivoco con la
                    // palabra por lo menos una vez
                    let easy = 1;
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
        println!("La palabra correcta es: {}", correct_answer.join(" / "));

        println!();
        println!("Ejemplo: {}", w.example_de);
        println!("Traducción: {}", w.example_es);
        println!();

        // Reproduces audio in case of failed
        player.play_from_path(
            manage_audio,
            hash_audios.get(&w.id),
            &w.gender_id,
            lang_second_audio,
        )?;

        loop {
            let Some(input) = ui::prompt_nonempty("> ")? else {
                break;
            };

            let input = input.trim().to_owned();
            match input.as_ref() {
                "q" | "exit" => {
                    val_out = 1;
                    break;
                }
                "1" => {
                    worte_correct.rotate_left(1); // mueve el primer elemento al final del vector
                    break;
                }
                "r" => {
                    player.play_from_path(
                        manage_audio,
                        hash_audios.get(&w.id),
                        &w.gender_id,
                        lang,
                    )?;
                }
                _ => {}
            }
        }
    }

    Ok((val_out, vec_out))
}
