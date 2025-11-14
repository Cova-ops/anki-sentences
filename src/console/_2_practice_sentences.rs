use color_eyre::eyre::Result;

use crate::{
    console::{
        _2_1_random_sentences::menu_2_1_random_sentences, _2_2_select_thema::menu_2_2_select_thema,
        _2_3_schwirig_sentences::menu_2_3_schwirig_sentences,
    },
    helpers::ui,
};

const TEXT_MENU: &str = r##"¿Cuál de entrenamiento quieres realizar?
  1. Oraciones aleatorias.
  2. Tema en especifico.
  3. Oraciones dificiles.
  4. Oraciones dificiles con tema.
  5. Solo errores anteriores.

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
            "2" => menu_2_2_select_thema()?,
            "3" => menu_2_3_schwirig_sentences()?,
            "4" => todo!(),
            "5" => todo!(),
            "exit" => return Ok(()),
            _ => println!("Comando no reconocido"),
        }
    }

    Ok(())
}
