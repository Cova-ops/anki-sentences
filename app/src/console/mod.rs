use color_eyre::eyre::Result;

use crate::{db::get_conn, helpers::ui};

mod _1_add_sentences;
mod _2_1_random_sentences;
mod _2_2_select_thema;
mod _2_3_schwirig_sentences;
mod _2_4_neue_setze;
mod _2_practice_sentences;
mod _3_add_worte;
mod _4_1_neue_worte;
mod _4_2_worte_review;
mod _4_practice_worte;
mod _5_2_audios_on_worte;
mod _5_manage_audios;

const MENU_MAIN_TEXT: &str = r#"
Herzliche Willkommen zu meinem Programm.
📋 Das Menu:
    1.- Hinzufügen neue Sätze.
    2.- Üben Sätze.
    3.- Hinzufügen neue Worte.
    4.- Üben Worte.
    5.- Manage Audios.

Para salir favor de escribir "exit"
"#;

pub fn menu_main() -> Result<()> {
    // clean_screen();

    let mut conn = get_conn();
    loop {
        println!("{}", MENU_MAIN_TEXT);
        let Some(input) = ui::prompt_nonempty("> ")? else {
            break;
        };

        // clean_screen();
        match input.trim() {
            "1" => _1_add_sentences::menu_1_add_sentences(&mut conn)?,
            "2" => _2_practice_sentences::menu_2_practice_sentences(&mut conn)?,
            "3" => _3_add_worte::menu_3_add_worte(&mut conn)?,
            "4" => _4_practice_worte::menu_4_practice_worte(&mut conn)?,
            "5" => _5_manage_audios::menu_5_manage_audios(&mut conn)?,
            "exit" => return Ok(()),
            _ => println!("Comando no reconocido"),
        };
    }

    println!("Muchas de nadas, vuelva pronto! 🙌");
    Ok(())
}
