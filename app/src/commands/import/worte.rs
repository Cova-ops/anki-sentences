use std::{
    env,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{self, Result};

use crate::{
    console::cli::TypeFile,
    db::{
        get_conn, schemas::worte_audio::NewWorteAudioSchema, worte::WorteRepo,
        worte_audio::WorteAudioRepo,
    },
    helpers::{self, audios::ManageAudios, csv, toml::AppConfig, ui},
    services::tts::{self, eleven_labs::LanguageVoice},
    utils::{self, path::path_to_string},
};

const TEXT_INSTRUCTIONS: &str = r##"
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
    3. Para los campos worte_de y worte_es no debes poner el articulo de los sustantivos, solo el sustantivo en cuestion. Ejemplo:
        - "der Tisch" - ERROR
        - "Tisch" - Correcto

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

pub fn run<P>(config: &AppConfig, path: P, type_file: TypeFile) -> Result<()>
where
    P: AsRef<Path>,
{
    let ext_valid = &["csv"];

    utils::path::validate_save_filename(&path, ext_valid)?;

    let base_dir = env::current_dir()?;
    let path_file = base_dir.join(path);

    let new_data = match type_file {
        TypeFile::CSV => csv::extract_worte_csv(&path_file)?,
        TypeFile::JSON => todo!("Aguantame papito"),
    };

    println!();
    println!("Procesando {} nuevas palabras.", new_data.len());
    let mut conn = get_conn(config.get_database_path()?)?;

    let res = WorteRepo::bulk_insert(&mut conn, &new_data)?;

    if !config.is_audio_enable()? {
        println!("The Worte are added, audio download is disable");
        return Ok(());
    }

    println!("The Worte are added, audio download starts");
    let manage_audios = ManageAudios::new(
        config.get_path_audios_worte()?,
        config.get_path_audios_setze()?,
    );

    for (i, wort) in res.iter().enumerate() {
        let audio_bytes: Vec<u8> =
            match tts::eleven_labs::generate_tts(&wort.worte_es, LanguageVoice::Spanisch) {
                Ok(v) => v,
                Err(err) => {
                    println!("Error al generar el TTS de la palabra: {}", wort.worte_es);
                    println!("{:#?}", err);
                    continue;
                }
            };

        let audio_path = match manage_audios.save_audio_worte(audio_bytes, wort.id) {
            Ok(v) => v,
            Err(err) => {
                println!("Error al guardar el archivo: {}", wort.worte_es);
                println!("{:#?}", err);
                continue;
            }
        };

        WorteAudioRepo::bulk_insert(
            &mut conn,
            &[NewWorteAudioSchema {
                wort_id: wort.id,
                voice_id: "masc_eleven_labs".into(),
                file_path: utils::path::path_to_string(&audio_path)?,
            }],
        )?;

        println!("Audio completed {}/{}.", i + 1, new_data.len());
    }

    Ok(())
}
