use color_eyre::eyre::Result;
use rusqlite::Connection;

use crate::{console::_5_2_audios_on_worte::menu_5_2_audios_on_worte, helpers::ui};

const TEXT_MENU: &str = r##"¿Cuál entrenamiento quieres realizar?
  1. Borrar audios no ocupados.
  2. Validar que las palabras tengan Audio.
  3. Validar que las oraciones tengan Audio.

Para regresar al menúprincipal favor de escribir "exit".
"##;

pub fn menu_5_manage_audios(conn: &mut Connection) -> Result<()> {
    // clean_screen();

    loop {
        println!("{}", TEXT_MENU);

        let Some(input) = ui::prompt_nonempty("> ")? else {
            break;
        };
        let input = input.trim();

        match input {
            "2" => menu_5_2_audios_on_worte(conn)?,
            "exit" => return Ok(()),
            _ => println!("Comando no reconocido!!"),
        }
    }

    Ok(())
}
