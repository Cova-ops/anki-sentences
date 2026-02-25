use std::collections::{HashMap, HashSet};

use crate::{
    console::cli::ReviewWorteSection,
    db::{
        get_conn,
        schemas::{
            wort_audio::ModelWortAudio,
            wort_review::{EnumReviewDirection, InputWortReview, ModelWortReview},
        },
        wort_audio::WortAudioRepo,
        wort_review::WortReviewRepo,
    },
    helpers::{self, error_handler::AppError, review_state::ReviewState, toml::AppConfig},
    services::tts::eleven_labs::LanguageVoice,
    utils,
};

use chrono::Utc;
use rand::seq::SliceRandom;
use rusqlite::Connection;

enum TypeExercise {
    Write,
    Speak,
}

fn get_review_new_ids(
    conn: &Connection,
    date_review: chrono::DateTime<Utc>,
    lang: LanguageVoice,
) -> Result<Vec<i32>, AppError> {
    let lang: EnumReviewDirection = lang.into();

    let mut vec_ids =
        WortReviewRepo::fetch_review_wort_id_by_day(&conn, date_review, lang.clone())?;

    vec_ids.append(&mut WortReviewRepo::fetch_new_wort_id_4_review(
        &conn, lang,
    )?);
    vec_ids.sort_unstable();
    vec_ids.dedup();

    Ok(vec_ids)
}

pub fn run(
    config: &AppConfig,
    section: ReviewWorteSection,
    batch: usize,
    no_shuffle: bool,
    lang: LanguageVoice,
) -> Result<(), AppError> {
    let mut conn = get_conn(config.get_database_path()?)?;

    let today = helpers::time::utc_datetime(1);
    let review_direction: EnumReviewDirection = lang.into();

    let mut ids_worte: Vec<i32> = match section {
        ReviewWorteSection::NewAndReview => get_review_new_ids(&conn, today, lang)?,
        ReviewWorteSection::OnlyNew => {
            WortReviewRepo::fetch_new_wort_id_4_review(&conn, review_direction)?
        }
        ReviewWorteSection::OnlyReview => {
            WortReviewRepo::fetch_review_wort_id_by_day(&conn, today, review_direction)?
        }
        _ => todo!("Aguantame papito"),
    };

    let type_exercise = match lang {
        LanguageVoice::Deutsch => TypeExercise::Speak,
        LanguageVoice::Spanisch => TypeExercise::Write,
    };

    let vec: Vec<_> = if config.is_audio_enable()? {
        WortAudioRepo::fetch_by_id(&conn, &ids_worte)?
    } else {
        Vec::new()
    };
    let ids_audios: Vec<_> = ModelWortAudio::try_from_iter(vec)?;
    let hash_audios: HashSet<i32> = ids_audios.iter().map(|ia| ia.wort_id).collect();

    if no_shuffle {
        let mut rng = rand::rng();
        ids_worte.shuffle(&mut rng);
    }

    let manage_audio = helpers::audios::ManageAudios::new(
        config.get_path_audios_worte()?,
        config.get_path_audios_setze()?,
        config.get_path_audios_artikel()?,
    );

    let result_review = match type_exercise {
        TypeExercise::Write => helpers::console::make_worte_exercise_write(
            &conn,
            ids_worte,
            hash_audios,
            &manage_audio,
            batch,
            no_shuffle,
            lang,
        )?,
        TypeExercise::Speak => helpers::console::make_worte_exercise_speak(
            &conn,
            ids_worte,
            hash_audios,
            &manage_audio,
            batch,
            no_shuffle,
            lang,
        )?,
    };
    // Obtenemos el id de las palabras que respondio
    let wort_ids: Vec<i32> = result_review.1.iter().map(|(id, _)| *id).collect();

    // Obtenemos si estas palabras ya tenian informacion hsitorica de revisiones anteriores
    let vec: Vec<_> = WortReviewRepo::fetch_by_wort_id(&conn, &wort_ids)?;
    let vec_worte_review: Vec<_> = ModelWortReview::try_from_iter(vec)?;

    let hash_worte_review: HashMap<i32, ModelWortReview> = vec_worte_review
        .into_iter()
        .filter(|f| f.direction == EnumReviewDirection::from(lang))
        .map(|wr| (wr.wort_id, wr))
        .collect();

    let mut vec_new_worte_review: Vec<InputWortReview> = vec![];
    let now = Utc::now();

    // Recorremos el arreglo de palabras que respondio el usuario
    for wort in result_review.1 {
        let wort_id = wort.0;
        let quality = wort.1;

        // Si tiene historico de revisiones usamos esa info, si no creamos un nuevo struct
        let review_state = if let Some(val) = hash_worte_review.get(&wort_id) {
            ReviewState::from(val.interval, val.ease_factor, val.repetitions)
        } else {
            ReviewState::new()
        };

        // generamos el arreglo para guardar las revisiones para un futuro
        let review_state = review_state.review(quality);
        let next = review_state.next_review_date_from(now);
        vec_new_worte_review.push(InputWortReview {
            wort_id,
            direction: EnumReviewDirection::from(lang),
            interval: review_state.interval,
            ease_factor: review_state.ease_factor,
            repetitions: review_state.repetitions,
            last_review: now,
            next_review: next,
        })
    }

    // guardamos en db la info de las revisiones
    WortReviewRepo::bulk_upsert(&mut conn, &vec_new_worte_review)?;

    if result_review.0 == 1 {
        return Ok(());
    }

    utils::console::clean_screen();
    println!("No hay mas palabras por estudiar. :)");
    println!();

    Ok(())
}
