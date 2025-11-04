use std::io::{self, BufRead, BufReader};

use color_eyre::eyre::{Context, Result};

use crate::{
    db::SetzeRepo::fetch_random,
    utils::{clean_screen, clean_sentences},
};

const TEXT_MENU: &str = r##"
Para salir pon la palara "exit".
Algunas letras que te pueden ayudar. :)
          - ß ẞ ä ö ü Ä Ö Ü 

  Oración: {satz}

Por favor traducela...
"##;

pub fn menu_2_1_random_sentences() -> Result<()> {
    // clean_screen();

    loop {
        let setze = fetch_random(100)?;
        let stdin = io::stdin();
        let mut reader = stdin.lock(); // StdinLock implementa BufRead
        let mut buffer = Vec::new();

        for s in setze {
            clean_screen();
            let mut s_done = false;
            let db_s = clean_sentences(&s.setze_deutsch);
            for _ in 0..3 {
                println!("{}", TEXT_MENU.replace("{satz}", &s.setze_spanisch));

                buffer.clear(); // importante: limpia el buffer entre lecturas
                reader
                    .read_until(b'\n', &mut buffer)
                    .context("[menu_2_practice_sentences] - Error al recibir el input")?;

                let input = String::from_utf8_lossy(&buffer).trim().to_string();
                if input == "exit" {
                    return Ok(());
                }

                let input = clean_sentences(&input);

                println!("Oración de DB: {}", db_s);
                println!("Oración del usuairo: {}", input);
                if input == db_s {
                    println!("Oración perfecta.");
                    s_done = true;
                    // TODO: Agregar el registro a DB
                    break;
                } else {
                    // clean_screen();
                    println!("Oración incorrecta");
                    println!("{}", TEXT_MENU.replace("{satz}", &s.setze_spanisch));
                }
            }

            if !s_done {
                println!("La oración correcta es: {}", s.setze_deutsch);
                println!("Schreib es gut, bitte.");

                loop {
                    buffer.clear(); // importante: limpia el buffer entre lecturas
                    reader
                        .read_until(b'\n', &mut buffer)
                        .context("[menu_2_practice_sentences] - Error al recibir el input")?;

                    let input = String::from_utf8_lossy(&buffer).trim().to_string();
                    if input == "exit" {
                        return Ok(());
                    }
                    let input = clean_sentences(&input);

                    if input == db_s {
                        break;
                    }
                }
            }
        }
    }
}
