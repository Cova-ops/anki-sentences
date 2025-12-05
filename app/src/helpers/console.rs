use std::collections::{HashMap, HashSet};

use color_eyre::eyre::Result;
use rusqlite::Connection;

use crate::{
    db::{
        schemas::{
            geschichtlich_setze::NewGeschichtlichSetzeSchema, setze::SetzeSchema,
            worte::WorteSchema,
        },
        worte::WorteRepo,
    },
    helpers::ui,
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
Para salir pon la palara "exit".
Algunas letras que te pueden ayudar. :)
          - ß ẞ ä ö ü Ä Ö Ü 

  Tema: {thema}
  Oración: {satz}

Por favor traducela...
"##;

/// return:
/// - 0 Finishing sentences
/// - 1 User typed "exit"
pub fn make_setze_exercise(arr: &[SetzeSchema]) -> Result<(u8, Vec<NewGeschichtlichSetzeSchema>)> {
    let mut vec_out = Vec::with_capacity(arr.len());
    let mut val_out = 0;

    for s in arr {
        utils::clean_screen();
        let mut s_done = false;

        let db_s = utils::string::clean_sentences(&s.setze_deutsch);
        for i in 0..2 {
            println!(
                "{}",
                TEXT_SETZE_ONCE
                    .replace("{satz}", &s.setze_spanisch)
                    .replace("{thema}", &s.thema)
            );

            let Some(input) = ui::prompt_nonempty("> ")? else {
                continue;
            };

            if input == "exit" {
                val_out = 1;
                break;
            }

            let input = utils::string::clean_sentences(&input);
            if input == db_s {
                println!("Oración perfecta.");
                s_done = true;

                // Si "i" vale 0, quiere decir que respondio al oración a la primera,
                // se pasa un true
                let new_data = NewGeschichtlichSetzeSchema {
                    setze_id: s.id,
                    result: i == 0,
                };
                vec_out.push(new_data);

                break;
            } else {
                println!();
                println!("Oración incorrecta");
            }
        }

        if !s_done && val_out != 1 {
            println!("La oración correcta es: {}", s.setze_deutsch);
            println!("Schreib es gut, bitte.");

            loop {
                let Some(input) = ui::prompt_nonempty("> ")? else {
                    break;
                };
                if input == "exit" {
                    val_out = 1;
                    break;
                }

                let input = utils::string::clean_sentences(&input);
                if input == db_s {
                    let new_data = NewGeschichtlichSetzeSchema {
                        setze_id: s.id,
                        result: false,
                    };
                    vec_out.push(new_data);
                    break;
                }
            }
        }
    }

    Ok((val_out, vec_out))
}

/// return:
/// - 0 Finishing sentences
/// - 1 User typed "exit"
pub fn make_setze_exercise_repeat(
    arr: &[SetzeSchema],
) -> Result<(u8, Vec<NewGeschichtlichSetzeSchema>)> {
    let mut setze_correct: Vec<SetzeSchema> = Vec::from(arr);
    let mut i = 0;

    let mut vec_out = Vec::with_capacity(arr.len());
    let mut val_out = 0;
    while !setze_correct.is_empty() {
        let s = setze_correct[i].clone();

        utils::clean_screen();
        let mut s_done = false;
        let db_s = utils::string::clean_sentences(&s.setze_deutsch);
        for i in 0..2 {
            println!(
                "{}",
                TEXT_SETZE_ONCE
                    .replace("{satz}", &s.setze_spanisch)
                    .replace("{thema}", &s.thema)
            );

            let Some(input) = ui::prompt_nonempty("> ")? else {
                continue;
            };

            if input == "exit" {
                val_out = 1;
                break;
            }

            let input = utils::string::clean_sentences(&input);
            if input == db_s {
                println!("Oración perfecta.");
                s_done = true;

                // Si "i" vale 0, quiere decir que respondio al oración a la primera,
                // se pasa un true
                if i == 0 {
                    let new_data = NewGeschichtlichSetzeSchema {
                        setze_id: s.id,
                        result: true,
                    };
                    vec_out.push(new_data);
                    setze_correct.remove(i);
                }

                break;
            } else {
                println!();
                println!("Oración incorrecta");
            }
        }

        if !s_done && val_out != 1 {
            println!("La oración correcta es: {}", s.setze_deutsch);
            println!("Schreib es gut, bitte.");

            loop {
                let Some(input) = ui::prompt_nonempty("> ")? else {
                    break;
                };
                if input == "exit" {
                    val_out = 1;
                    break;
                }

                let input = utils::string::clean_sentences(&input);
                if input == db_s {
                    let new_data = NewGeschichtlichSetzeSchema {
                        setze_id: s.id,
                        result: false,
                    };
                    vec_out.push(new_data);
                    break;
                }
            }
        }

        i += 1;
        i %= arr.len();
    }

    Ok((val_out, vec_out))
}

const TEXT_WORTE_ONCE: &str = r##"
Para salir pon la palara "exit".
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
    offset: usize,
) -> Result<(i32, Vec<(i32, u8)>)> {
    let mut ids_worte = ids_worte;

    let mut vec_out: Vec<(i32, u8)> = vec![];
    let mut val_out = 0;
    let mut already_studied: HashMap<i32, ManageRepetitions> = HashMap::new();

    let take = ids_worte.len().min(offset);
    let aux_ids: Vec<i32> = ids_worte.drain(..take).collect();

    // Obtenemos toda la info del bloque de palabras que vamos a usar
    let mut worte_correct = WorteRepo::fetch_by_id(conn, &aux_ids)?;

    while !worte_correct.is_empty() && val_out == 0 {
        let w = worte_correct[0].clone();

        utils::clean_screen();
        // println!(
        //     "hash: {:#?}",
        //     already_studied
        //         .iter()
        //         .map(|a| format!("{} {:#?}", a.0, a.1))
        //         .collect::<Vec<_>>()
        //         .join("\n")
        // );
        // println!("w: {:#?}", w);
        // println!("worte_correct: {:#?}", worte_correct);
        println!(
            "{}",
            TEXT_WORTE_ONCE.replace("{wort}", &w.worte_es).replace(
                "{gram_type}",
                &w.gram_type_id
                    .into_iter()
                    .map(|r| format!("{} ", r.name))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        );

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
            println!("Palabra perfecta.");

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

                    // Consultamos una nueva palabra y la añadimos al arreglo para su estudio
                    let id_new = ids_worte.remove(0);
                    let wort_new = WorteRepo::fetch_by_id(conn, &[id_new])?;
                    worte_correct.push(wort_new[0].clone());

                    // limpiamos el hashmap de la palabra que ya no se va a repetir
                    already_studied.remove(&w.id);
                }
            } else {
                already_studied.insert(w.id, ManageRepetitions::new());
                worte_correct.rotate_left(1); // mueve el primer elemento al final del vector
            }

            continue;
        }

        already_studied
            .entry(w.id)
            .and_modify(|r| *r = ManageRepetitions::new_error())
            .or_insert(ManageRepetitions::new_error());

        println!();
        println!("Oración incorrecta");
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
