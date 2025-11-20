use color_eyre::eyre::Result;
use rusqlite::Connection;

use crate::{
    db::worte::WorteRepo,
    helpers::{extract_worte_csv, is_csv_valid, ui},
    utils::path_file_oder_dir,
};

const TEXT_MENU: &str = r##"
INSTRUCCIONES PARA AGREGAR UN NUEVO ARCHIVO DE PALABRAS (CSV)

El archivo debe ser un CSV con la siguiente estructura de columnas (en este orden exacto):

Cabeceras:
    - gram_type (Tipo de palabra)
    - gender_id (si no tiene dejar vacio)
    - worte_de (Palabra en alemán)
    - worte_es (Palabra en español)
    - plural (si no tiene dejar vacio)
    - niveau_id (Nivel de la palabra)
    - example_de (Ejemplo usando la palabra en aleman)
    - example_es (Ejemplo usando la palabra en español)
    
    (Solo para verbos, si no aplica dejar en vacio)
    - verb_aux (Verbo auxiliar que requiere)
    - trennbar (Si / No. Si es un verbo separable)
    - reflexiv (Si / No. Si es un verbo reflexivo)

gram_type:
	- noun_common (Sustantivo comun)
	- noun_proper (Nombre propio)
	- verb_main (Verbo lexico)
	- verb_modal (Verbo modal)
	- verb_auxiliary (Verbo auxiliar)
	- verb_separable (Verbo separable)
	- verb_reflexive (Verbo reflexivo)
	- adjective (Adjetivo)
	- adverb_time (Adverbio tiempo)
	- adverb_place (Adverbio lugar)
	- adverb_manner (Adverbio modo)
	- adverb_degree (Adverbio grado)
	- adverb_sentence_connector (Adverbio conector)
	- pronoun_personal (Pronombre personal)
	- pronoun_possessive (Pronombre posesivo)
	- pronoun_reflexive (Pronombre reflexivo)
	- pronoun_demonstrative (Pronombre demostrativo)
	- pronoun_relative (Pronombre relativo)
	- pronoun_interrogative (Pronombre interrogativo)
	- pronoun_indefinite (Pronombre indefinido)
	- article_definite (Articulo definido)
	- article_indefinite (Articulo indefinido)
	- determiner_quantifier (Determinante cuantificador)
	- preposition_dative (Preposicion dativo)
	- preposition_akkusative (Preposicion acusativo)
	- preposition_genitive (Preposicion genitivo)
	- preposition_two_way (Preposicion doble)
	- conjunction_coordinating (Conjuncion coordinante)
	- conjunction_subordinating (Conjuncion subordinante)
	- particle_modal (Particula modal)
	- particle_focus (Particula enfoque)
	- particle_negation (Particula negacion)
	- particle_answer (Particula respuesta)
	- numeral_cardinal (Numeral cardinal)
	- numeral_ordinal (Numeral ordinal)
	- interjection (Interjeccion)
	- fixed_phrase (Frase fija)
	- prefix_separable (Prefijo separable)
	- pattern_verb_dativ (Patron verbo dativo)
	- pattern_verb_akkusativ (Patron verbo acusativo)
	- pattern_verb_dat_akk (Patron verbo dativo-acusativo)

gender_id: 
    - Maskuline
	- Femenin
	- Neutrum
	- Plural

niveau_id:
    - A1
	- A2
	- B1
	- B2
	- C1
	- C2

Consideraciones:
    1. Poner las oraciones entre comillas dobles, en caso de tener "," dentro de las mismas.
    2. Para el gram_type, si hay palabras que pueden tener mas de una se debera poner separadas por coma ','. Ejemplo:
        ...","particle_modal,interjection","...

Ejemplo:
gram_type,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv
noun_common,Maskuline,der Hund,el perro,Hunde,A1,"Der Hund spielt im Garten.","El perro juega en el jardín.",,,
verb_main,,gehen,ir,,A1,"Ich gehe heute früh zur Arbeit.","Hoy voy temprano al trabajo.","sein","No","No"
verb_separable,,"anrufen",llamar por teléfono,,A2,"Ich rufe meine Freundin später an.","Llamo a mi amiga más tarde.","haben","Si","No"
verb_reflexive,,"sich freuen",alegrarse,,A2,"Ich freue mich über das Geschenk.","Me alegro por el regalo.","haben","No","Si"

Para poder agregar el archivo por favor pon la ruta donde se encuentra tu CSV. La ruta puede ser en formato relativo o absoluto. Ejemplo:
    - Abs: ../archivo.csv
    - Rel: /Users/daniel/Documents/Programas/git/anki-sentences/data/text.csv

Para regresar al menu principal favor de escribir "exit".
"##;

const TEXT_ERROR_FILE_WRONG: &str = r##"
El archivo proporcionado no existe o no es un archivo CSV válido.
"##;

// TODO: Probar que esta cosa funcione bien
pub fn menu_3_add_worte(conn: &mut Connection) -> Result<()> {
    // clean_screen();

    println!("{}", TEXT_MENU);

    let mut csv_path: String;
    loop {
        let Some(input) = ui::prompt_nonempty("> ")? else {
            break;
        };

        if input == "exit" {
            return Ok(());
        }

        csv_path = input.clone();
        let mut err_2_show: Option<&str> = None;
        let valid_1 = path_file_oder_dir(&input);

        // No existe la dirección ó no es un archivo
        if valid_1.is_err() || !valid_1?.0 {
            err_2_show = Some(TEXT_ERROR_FILE_WRONG);
        }

        let valid_2 = is_csv_valid(&input);
        if valid_2.is_err() && err_2_show.is_none() {
            err_2_show = Some(TEXT_ERROR_FILE_WRONG);
        }

        if err_2_show.is_some() {
            println!("{}", err_2_show.unwrap());
            continue;
        }

        let new_data = extract_worte_csv(&csv_path)?;
        let res = WorteRepo::bulk_insert(conn, &new_data);

        if res.is_err() {
            println!("Ups ha ocurrido un error: {:#?}", res);
            println!("Favor de corregirlo e intentar de nuevo");
            continue;
        }

        break;
    }

    //
    // println!("{:#?}", SetzeRepo::fetch_random(100)?);

    Ok(())
}
