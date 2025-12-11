use color_eyre::eyre::Result;
use rusqlite::Connection;

use crate::{
    console::{_4_1_neue_worte::menu_4_1_neue_worte, _4_2_worte_review::menu_4_2_worte_review},
    helpers::ui,
};

const TEXT_MENU: &str = r##"¿Cuál entrenamiento quieres realizar?
  1. Palabras nuevas.
  2. Repetición de palabras.

Para regresar al menú principal favor de escribir "exit".
"##;

pub fn menu_4_practice_worte(conn: &mut Connection) -> Result<()> {
    // clean_screen();

    loop {
        println!("{}", TEXT_MENU);

        let Some(input) = ui::prompt_nonempty("> ")? else {
            break;
        };
        let input = input.trim();

        match input {
            "1" => menu_4_1_neue_worte(conn)?,
            "2" => menu_4_2_worte_review(conn)?,
            "exit" => return Ok(()),
            _ => println!("Comando no reconocido!!"),
        }
    }

    Ok(())
}
