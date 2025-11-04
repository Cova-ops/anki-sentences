use std::io;

use color_eyre::eyre::{Context, Result};

use crate::{
    db::SetzeRepo,
    helpers::{extract_sentences_csv, is_csv_valid},
    utils::{clean_screen, path_file_oder_dir},
};

const TEXT_MENU: &str = r##"
Para agregar un nuevo archivo de oraciones se debe de hacer bajo un archivo tipo CSV con el siguiente formato dentro del mismo.
Cabeceras:
    - setze_spanisch (Oración en español) 
    - setze_deutsch (Oración en alemán)
    - thema (Tema al que corresponde la oración)
    - schwirig_id (Calificación de la oración de dificultad, puede ser con número o letra)

schwirig_id:
    - 0: "Einfag"
    - 1: "Normal"
    - 2: "Schwirig"

Ejemplo:

setze_spanisch,setze_deutsch,thema,schwirig_id
"Tengo un perro","Ich habe einen Hund","Akkusativ",0
"Voy con mi gato hacia Alemania","Ich fahre mit meiner Katze nach Deutschland","Präpositionen mit Dativ","Normal"

Recomendaciones:
    1. Poner las oraciones entre comillas dobles, en caso de tener "," dentro de las mismas.

Para poder agregar el archivo por favor pon la ruta donde se encuentra tu CSV.
Para regresar al menu principal favor de escribir "exit".
"##;

const TEXT_ERROR_FILE_NO_EXIST: &str = r##"
La ruta seleccionada no corresponde a un archivo de formato CSV.
Favor de colocar la ruta en formato relativo o absulto. Ejemplo:
    - Abs: ../archivo.csv
    - Rel: /Users/daniel/Documentos/archivo.csv
    - Rel: /Users/daniel/Documents/Programas/git/anki-sentences/data/text.csv

Para regresar al menu principal favor de escribir "exit".
"##;

const TEXT_ERROR_FILE_NO_CSV: &str = r##"
La ruta seleccionada no corresponde a un archivo de formato CSV.
Favor de colocar la ruta en formato relativo o absulto. Ejemplo:
    - Abs: ../archivo.csv
    - Rel: /Users/daniel/Documentos/archivo.csv

Recuerda que el archivo debe de tener este formato:

setze_spanisch,setze_deutsch,schwirig_id
"Tengo un perro","Ich habe einen Hund",0
"Voy con mi gato hacia Alemania","Ich fahre mit meiner Katze nach Deutschland",Normal

Para regresar al menu principal favor de escribir "exit".
"##;

pub fn menu_1_add_sentences() -> Result<()> {
    // clean_screen();

    println!("{}", TEXT_MENU);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .context("[menu_1_add_sentences] - Error al recibir el input")?;

    input = input.trim().to_string();
    if input == "exit" {
        return Ok(());
    }

    loop {
        let mut err_2_show: Option<&str> = None;
        let valid_1 = path_file_oder_dir(&input);

        // No existe la dirección ó no es un archivo
        if valid_1.is_err() || !valid_1.ok().unwrap().0 {
            err_2_show = Some(TEXT_ERROR_FILE_NO_EXIST);
        }

        let valid_2 = is_csv_valid(&input);
        if valid_2.is_err() && err_2_show.is_none() {
            err_2_show = Some(TEXT_ERROR_FILE_NO_CSV);
        }

        if err_2_show.is_none() {
            break;
        }

        // clean_screen();
        // println!("valid_1: {:?}", valid_1.is_err());
        println!("{}", err_2_show.unwrap());
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .context("[menu_1_add_sentences] - Error al leer la línea")?;

        input = input.trim().to_string();
        if input == "exit" {
            return Ok(());
        }
    }

    let new_data = extract_sentences_csv(&input)?;
    SetzeRepo::bulk_insert(new_data)?;
    //
    // println!("{:#?}", SetzeRepo::fetch_random(100)?);

    Ok(())
}
