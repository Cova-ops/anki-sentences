use color_eyre::eyre::Result;

use crate::{
    console::{
        _1_add_sentences::menu_1_add_sentences, _2_practice_sentences::menu_2_practice_sentences,
        _3_add_worte::menu_3_add_worte,
    },
    db::get_conn,
    helpers::ui,
};

const MENU_MAIN_TEXT: &str = r#"
Herzliche Willkommen zu meinem Programm.
📋 Das Menu:
    1.- Hinzufügen neue Sätze.
    2.- Üben neue Sätze.
    3.- Hinzufügen neue Worte.
    4.- Üben Artikel Geschlecht.

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
            "1" => menu_1_add_sentences(&mut conn)?,
            "2" => menu_2_practice_sentences(&mut conn)?,
            "3" => menu_3_add_worte(&mut conn)?,
            "4" => todo!(),
            "exit" => return Ok(()),
            _ => println!("Comando no reconocido"),
        }
    }

    println!("Muchas de nadas, vuelva pronto! 🙌");
    Ok(())
}
