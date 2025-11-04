use std::io;

use color_eyre::eyre::{Context, Result};

use crate::{console::_2_1_random_sentences::menu_2_1_random_sentences, helpers::ui};

const TEXT_MENU: &str = r##"
Que tipo de entrenamiento quieres realizar?

  1. Oraciones aleatorias.
  2. Solo errores anteriore.
  3. Tema en especifico.

Para regresar al menu principal favor de escribir "exit".
"##;

pub fn menu_2_practice_sentences() -> Result<()> {
    // clean_screen();

    loop {
        println!("{}", TEXT_MENU);
        let Some(input) = ui::prompt_nonempty("> ")? else {
            break;
        };

        if input == "exit" {
            return Ok(());
        }

        match input.trim() {
            "1" => menu_2_1_random_sentences()?,
            "2" => todo!(),
            "3" => todo!(),
            "exit" => return Ok(()),
            _ => println!("Comando no reconocido"),
        }
    }

    Ok(())
}
