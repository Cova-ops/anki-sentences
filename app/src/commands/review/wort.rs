use std::collections::{HashMap, HashSet};

use crate::{
    console::cli::ReviewWorteSection,
    db::{
        get_conn,
        schemas::{
            wort_audio::{ModelWortAudio, SchemaWortAudio},
            wort_review::{EnumReviewDirection, InputWortReview, ModelWortReview},
        },
        wort_audio::WortAudioRepo,
        wort_review::WortReviewRepo,
    },
    helpers::{
        self, audios::ManageAudios, error_handler::AppError, review_state::ReviewState,
        toml::AppConfig,
    },
    services::tts::language_voice::LanguageVoice,
    utils,
};

use chrono::Utc;
use rand::seq::SliceRandom;
use rusqlite::Connection;

enum TypeExercise {
    Write,
    Speak,
}

/// Get all ids from words that are new and the ones that need review
fn get_review_new_ids(
    conn: &Connection,
    date_review: chrono::DateTime<Utc>,
    lang: LanguageVoice,
) -> Result<Vec<i32>, AppError> {
    use WortReviewRepo as Repo;

    let lang: EnumReviewDirection = lang.into();

    let mut ids_reviews: Vec<i32> = Repo::fetch_review_wort_id_by_day(&conn, date_review, lang)?;
    let mut ids_news: Vec<i32> = Repo::fetch_new_wort_id_4_review(&conn, lang)?;

    let mut vec_out: Vec<i32> = Vec::with_capacity(ids_reviews.len() + ids_news.len());
    vec_out.append(&mut ids_reviews);
    vec_out.append(&mut ids_news);

    vec_out.sort_unstable();

    // In theory is impossible that it has duplicated
    vec_out.dedup();

    Ok(vec_out)
}

/// This functions does:
/// - Get data from DB
/// -
pub fn run(
    config: &AppConfig,
    section: ReviewWorteSection,
    batch: usize,
    no_shuffle: bool,
    lang: LanguageVoice,
) -> Result<(), AppError> {
    let mut conn: rusqlite::Connection = get_conn(config.get_database_path()?)?;

    let today: chrono::DateTime<Utc> = helpers::time::utc_datetime(1);
    let review_direction: EnumReviewDirection = lang.into();

    let mut ids_worte: Vec<i32> = {
        use ReviewWorteSection as Review;
        use WortReviewRepo as Repo;

        match section {
            Review::NewAndReview => get_review_new_ids(&conn, today, lang)?,
            Review::OnlyNew => Repo::fetch_new_wort_id_4_review(&conn, review_direction)?,
            Review::OnlyReview => {
                Repo::fetch_review_wort_id_by_day(&conn, today, review_direction)?
            }
            _ => todo!("Aguantame papito"),
        }
    };

    let type_exercise: TypeExercise = match lang {
        LanguageVoice::Deutsch => TypeExercise::Speak,
        LanguageVoice::Spanisch => TypeExercise::Write,
    };

    let vec: Vec<SchemaWortAudio> = match config.is_audio_enable()? {
        true => WortAudioRepo::fetch_by_id(&conn, &ids_worte)?,
        false => vec![],
    };

    let ids_audios: Vec<ModelWortAudio> = ModelWortAudio::try_from_iter(vec)?;
    let hash_audios: HashSet<i32> = ids_audios.iter().map(|ia| ia.wort_id).collect();

    if no_shuffle {
        let mut rng = rand::rng();
        ids_worte.shuffle(&mut rng);
    }

    let manage_audio: ManageAudios = helpers::audios::ManageAudios::new(
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
    let vec_worte_review: Vec<ModelWortReview> = ModelWortReview::try_from_iter(vec)?;

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
        let review_state: ReviewState = match hash_worte_review.get(&wort_id) {
            Some(v) => ReviewState::from(v.interval, v.ease_factor, v.repetitions),
            _ => ReviewState::new(),
        };

        // We modify the algorithm with the new respond of the user
        let review_state: ReviewState = review_state.review(quality);
        let next_review: chrono::DateTime<Utc> = review_state.next_review_date_from(now);

        // generamos el arreglo para guardar las revisiones para un futuro
        vec_new_worte_review.push(InputWortReview {
            wort_id,
            direction: lang.into(),
            interval: review_state.interval(),
            ease_factor: review_state.ease_factor(),
            repetitions: review_state.repetitions(),
            last_review: now,
            next_review: next_review,
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
