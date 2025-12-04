use std::collections::HashSet;

use color_eyre::eyre::Result;

use crate::{
    db::schemas::{
        geschichtlich_setze::NewGeschichtlichSetzeSchema, setze::SetzeSchema, worte::WorteSchema,
    },
    helpers::ui,
    utils,
};

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
pub fn make_worte_exercise_repeat(arr: &[WorteSchema]) -> Result<(i32, Vec<(i32, u8)>)> {
    let mut worte_correct: Vec<WorteSchema> = Vec::from(arr);

    let mut vec_out: Vec<(i32, u8)> = Vec::with_capacity(arr.len());
    let mut val_out = 0;
    let mut already_studied: HashSet<i32> = HashSet::new();

    while !worte_correct.is_empty() && val_out == 0 {
        let w = worte_correct[0].clone();

        utils::clean_screen();
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

            // Si no lo contiene significa que es la primera vez que pasa la palabra, por lo tanto
            // lo tuvo correcto a la primero se pone un -> 2.
            // Si lo contiene entonces la palabra ya habia pasado antes para estudiar y se habia
            // equivocado, entonces se pone un -> 1
            let easy = if already_studied.contains(&w.id) {
                1
            } else {
                2
            };
            vec_out.push((w.id, easy));
            worte_correct.remove(0);
            continue;
        }

        already_studied.insert(w.id);
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
