use std::io;

use color_eyre::eyre::{Context, Result};

use crate::{
    console::{
        _1_add_sentences::menu_1_add_sentences, _2_practice_sentences::menu_2_practice_sentences,
    },
    utils::clean_screen,
};

const MENU_MAIN_TEXT: &str = "
Herzliche Willkommen zu meinem Programm.
📋 Das Menu:
    1.- Hinzufügen neue Sätze.
    2.- Üben neue Sätze.
    3.- Ver Estadisticas.

    9.- Salir.
";

pub fn menu_main() -> Result<()> {
    // clean_screen();

    let mut input = String::new();

    loop {
        println!("{}", MENU_MAIN_TEXT);

        io::stdin()
            .read_line(&mut input)
            .context("Error al leer la línea")?;

        // clean_screen();
        match input.trim() {
            "1" => menu_1_add_sentences()?,
            "2" => menu_2_practice_sentences()?,
            "3" => todo!(),
            "9" => break,
            _ => println!("Comando no reconocido"),
        }
    }

    println!("Muchas de nadas, vuelva pronto! 🙌");
    Ok(())
}
