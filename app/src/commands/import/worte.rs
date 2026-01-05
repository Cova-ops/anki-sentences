use std::{collections::HashMap, env, path::Path};

use color_eyre::eyre::{Context, OptionExt, Result, bail};

use crate::{
    console::cli::TypeFile,
    db::{
        get_conn,
        schemas::{
            niveau_liste::NiveauListeSchema,
            worte::{NewWorteSchema, WorteSchema},
            worte_audio::NewWorteAudioSchema,
            worte_gender::WorteGenderSchema,
        },
        worte::WorteRepo,
        worte_audio::WorteAudioRepo,
        worte_gram_type::WorteGramTypeRepo,
    },
    helpers::{self, audios::ManageAudios, csv, toml::AppConfig},
    services::tts::{self, eleven_labs::LanguageVoice},
    utils,
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

fn process_audio(
    text: &str,
    wort_id: i32,
    manage_audios: &ManageAudios,
    lang: LanguageVoice,
) -> Result<String> {
    let audio_bytes: Vec<u8> = tts::eleven_labs::generate_tts(text, lang)?;
    let audio_path = manage_audios.save_audio_worte(audio_bytes, wort_id, lang)?;
    let audio_name = utils::path::get_filename_from_path(&audio_path)?;

    Ok(audio_name)
}

fn try_audio(
    text_es: &str,
    text_de: &str,
    wort_id: i32,
    manage_audios: &ManageAudios,
) -> (Option<String>, Option<String>) {
    let path_es = match process_audio(text_es, wort_id, manage_audios, LanguageVoice::Spanisch)
        .wrap_err_with(|| {
            format!("processing wort_id={wort_id}, lang=LanguageVoice::Spanisch, text={text_es}")
        }) {
        Ok(name) => Some(name),
        Err(err) => {
            eprintln!("{:#?}", err);
            None
        }
    };

    let path_de = match process_audio(text_de, wort_id, manage_audios, LanguageVoice::Deutsch)
        .wrap_err_with(|| {
            format!("processing wort_id={wort_id}, lang=LanguageVoice::Deutsch, text={text_de}")
        }) {
        Ok(name) => Some(name),
        Err(err) => {
            eprintln!("{:#?}", err);
            None
        }
    };

    (path_es, path_de)
}

enum ManageWortRepeatedResponse {
    Skip,
    ReplaceData(i32),
    Cancel,
}

fn manage_wort_repeated(
    old: &WorteSchema,
    new: &NewWorteSchema,
) -> Result<ManageWortRepeatedResponse> {
    utils::console::clean_screen();

    // vec<(field, old, new)>
    let mut diffs: Vec<(String, String, String)> = vec![];

    fn fmt_gender(id: Option<i32>) -> Result<String> {
        Ok(match id {
            Some(id) => {
                let artikel = WorteGenderSchema::from_id(id)?.artikel;
                format!("{id} ({artikel})")
            }
            None => "<None>".to_owned(),
        })
    }

    let old_gender_id = old.gender_id.as_ref().map(|g| g.id); // Option<i32>
    let new_gender_id = new.gender_id; // Option<i32>

    if old_gender_id != new_gender_id {
        diffs.push((
            "gender ".to_owned(),
            fmt_gender(old_gender_id)?,
            fmt_gender(new_gender_id)?,
        ));
    }

    fn fmt_plural(s: Option<&str>) -> String {
        match s {
            Some(s) => {
                format!("{s}")
            }
            None => "<None>".to_owned(),
        }
    }

    let old_plural = old.plural.as_deref();
    let new_plural = new.plural.as_deref();

    if old_plural != new_plural {
        diffs.push((
            "plural".to_owned(),
            fmt_plural(old_plural),
            fmt_plural(new_plural),
        ));
    }

    fn fmt_niveau(id: i32) -> Result<String> {
        let niveau = NiveauListeSchema::from_id(id)?.niveau;
        Ok(format!("{id} ({niveau})"))
    }

    let old_niveau_id = old.niveau_id.id; // i32
    let new_niveau_id = new.niveau_id; // i32

    if old_niveau_id != new_niveau_id {
        diffs.push((
            "niveau".to_owned(),
            fmt_niveau(old_niveau_id)?,
            fmt_niveau(new_niveau_id)?,
        ));
    }

    if old.example_es != new.example_es {
        diffs.push((
            "example (ES)".to_owned(),
            old.example_es.to_owned(),
            new.example_es.to_owned(),
        ));
    }

    if old.example_de != new.example_de {
        diffs.push((
            "example (DE)".to_owned(),
            old.example_de.to_owned(),
            new.example_de.to_owned(),
        ));
    }

    let old_verb_aux = old.verb_aux.as_deref();
    let new_verb_aux = new.verb_aux.as_deref();

    fn fmt_verb_aux(s: Option<&str>) -> String {
        match s {
            Some(v) => v.to_owned(),
            None => "<None>".to_owned(),
        }
    }

    if old_verb_aux != new_verb_aux {
        diffs.push((
            "verb_aux".to_owned(),
            fmt_verb_aux(old_verb_aux),
            fmt_verb_aux(new_verb_aux),
        ));
    }

    let old_trennbar = old.trennbar.as_ref();
    let new_trennbar = new.trennbar.as_ref();

    fn fmt_trennbar(b: Option<&bool>) -> String {
        match b {
            Some(v) => v.to_string(),
            None => "<None>".to_owned(),
        }
    }

    if old_trennbar != new_trennbar {
        diffs.push((
            "trennbar".to_owned(),
            fmt_trennbar(old_trennbar),
            fmt_trennbar(new_trennbar),
        ));
    }

    let old_reflexiv = old.reflexiv.as_ref();
    let new_reflexiv = new.reflexiv.as_ref();

    fn fmt_reflexiv(b: Option<&bool>) -> String {
        match b {
            Some(v) => v.to_string(),
            None => "<None>".to_owned(),
        }
    }

    if old_reflexiv != new_reflexiv {
        diffs.push((
            "reflexiv".to_owned(),
            fmt_reflexiv(old_reflexiv),
            fmt_reflexiv(new_reflexiv),
        ));
    }

    // if it is exact the same as DB automatic ignore word
    if diffs.is_empty() {
        return Ok(ManageWortRepeatedResponse::Skip);
    }

    println!();
    println!("[WARNING] Duplicate word detected");
    println!();

    println!("Word key:");
    println!("\tES: {}", old.worte_es);
    println!("\tDE: {}", old.worte_de);
    println!("────────────────────────────────────────");
    println!("Changes:");

    for row in diffs {
        println!("• {}", row.0);
        println!("\told: {}", row.1);
        println!("\tnew: {}", row.2);
    }

    println!();
    println!("────────────────────────────────────────");
    println!("How do you want to handle this word?");
    println!("  [r] Replace existing data");
    println!("  [s] Skip this word");
    println!("  [q] Cancel import");

    loop {
        let input = helpers::ui::prompt_nonempty("> ")?.ok_or_eyre("prompt returned None")?;

        return match input.trim().to_lowercase().as_str() {
            "r" | "replace" => Ok(ManageWortRepeatedResponse::ReplaceData(old.id)),
            "q" | "quit" => Ok(ManageWortRepeatedResponse::Cancel),
            "s" | "skip" => Ok(ManageWortRepeatedResponse::Skip),
            _ => continue,
        };
    }
}

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

    let mut vec_new_worte: Vec<NewWorteSchema> = vec![];
    let mut vec_update_worte: Vec<(i32, NewWorteSchema)> = vec![];
    for n in new_data.into_iter() {
        let data = &[(n.worte_es.clone(), n.worte_de.clone())];
        let worte_repeated = WorteRepo::fetch_by_wort(&conn, data)?;

        if worte_repeated.is_empty() {
            vec_new_worte.push(n);
            continue;
        }

        let res = manage_wort_repeated(&worte_repeated[0], &n)?;

        match res {
            ManageWortRepeatedResponse::Cancel => return Ok(()),
            ManageWortRepeatedResponse::Skip => {}
            ManageWortRepeatedResponse::ReplaceData(id) => vec_update_worte.push((id, n)),
        };
    }

    // update words
    for (id, new_worte) in vec_update_worte {
        let tx = conn.transaction()?;
        WorteGramTypeRepo::delete_by_id_tx(&tx, &[id])?;
        WorteRepo::bulk_update_tx(&tx, &[(id, new_worte)])?;
        tx.commit()?;
    }

    // insert new words
    let res = WorteRepo::bulk_insert(&mut conn, &vec_new_worte)?;

    if !config.is_audio_enable()? {
        println!("The Worte are added/updated, audio download is disable");
        return Ok(());
    }

    let manage_audios = ManageAudios::new(
        config.get_path_audios_worte()?,
        config.get_path_audios_setze()?,
    );

    println!("The Worte are added/updated, audio download starts");
    for (i, wort) in res.iter().enumerate() {
        let (audio_name_es, audio_name_de) =
            try_audio(&wort.worte_es, &wort.worte_de, wort.id, &manage_audios);

        if audio_name_es.is_none() && audio_name_de.is_none() {
            continue;
        }

        WorteAudioRepo::bulk_insert(
            &mut conn,
            &[NewWorteAudioSchema {
                wort_id: wort.id,
                audio_name_es,
                audio_name_de,
            }],
        )?;

        println!("Audio completed {}/{}.", i + 1, res.len());
    }

    println!();
    println!("Audio download complete :).");
    println!();

    Ok(())
}
