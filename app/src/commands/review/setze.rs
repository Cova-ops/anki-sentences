use std::collections::{HashMap, HashSet};

use crate::{
    console::cli::ReviewSetzeSection,
    db::{
        get_conn,
        schemas::{
            setze_audio::ModelSetzeAudio,
            setze_review::{InputSetzeReview, ModelSetzeReview},
        },
        setze::SetzeRepo,
        setze_audio::SetzeAudioRepo,
        setze_review::SetzeReviewRepo,
    },
    helpers::{self, error_handler::AppError, review_state::ReviewState, toml::AppConfig},
    utils,
};

use chrono::Utc;
use rand::seq::SliceRandom;

pub fn run(
    config: &AppConfig,
    section: ReviewSetzeSection,
    batch: usize,
    no_shuffle: bool,
) -> Result<(), AppError> {
    let mut conn = get_conn(config.get_database_path()?)?;

    let mut ids_setze: Vec<i32> = match section {
        ReviewSetzeSection::OnlyNew => SetzeRepo::fetch_id_neue_sentences(&conn)?,
        _ => todo!("Aguantame papito"),
    };

    let ids_audios = if config.is_audio_enable()? {
        SetzeAudioRepo::fetch_by_id(&conn, &ids_setze)?
    } else {
        Vec::new()
    };
    let ids_audios = ModelSetzeAudio::try_from_iter(ids_audios)?;

    let hash_audios: HashSet<i32> = ids_audios.iter().map(|ia| ia.satz_id).collect();

    if !no_shuffle {
        let mut rng = rand::rng();
        ids_setze.shuffle(&mut rng);
    }

    let manage_audio = helpers::audios::ManageAudios::new(
        config.get_path_audios_worte()?,
        config.get_path_audios_setze()?,
        config.get_path_audios_artikel()?,
    );
    let r = helpers::console::make_setze_exercise_repeat(
        &conn,
        ids_setze,
        hash_audios,
        &manage_audio,
        batch,
    )?;

    // Obtenemos el id de las oraciones que respondio
    let setze_ids: Vec<i32> = r.1.iter().map(|(id, _)| *id).collect();

    // Obtenemos si estas oraciones ya tenian informacion hsitorica de revisiones anteriores
    let vec: Vec<_> = SetzeReviewRepo::fetch_by_satz_id(&conn, &setze_ids)?;
    let vec_setze_review: Vec<_> = ModelSetzeReview::try_from_iter(vec)?;

    let hash_setze_review: HashMap<i32, ModelSetzeReview> = vec_setze_review
        .into_iter()
        .map(|sr| (sr.satz_id, sr))
        .collect();

    let mut vec_new_setze_review: Vec<InputSetzeReview> = vec![];
    let now = Utc::now();

    // Recorremos el arreglo de palabras que respondio el usuario
    for satz in r.1 {
        let satz_id = satz.0;
        let quality = satz.1;

        // Si tiene historico de revisiones usamos esa info, si no creamos un nuevo struct
        let review_state = if let Some(val) = hash_setze_review.get(&satz_id) {
            ReviewState::from(val.interval, val.ease_factor, val.repetitions)
        } else {
            ReviewState::new()
        };

        // generamos el arreglo para guardar las revisiones para un futuro
        let review_state = review_state.review(quality);
        let next = review_state.next_review_date_from(now);
        vec_new_setze_review.push(InputSetzeReview {
            satz_id,
            interval: review_state.interval,
            ease_factor: review_state.ease_factor,
            repetitions: review_state.repetitions,
            last_review: now,
            next_review: next,
        })
    }

    // guardamos en db la info de las revisiones
    SetzeReviewRepo::bulk_upsert(&mut conn, &vec_new_setze_review)?;

    if r.0 == 1 {
        return Ok(());
    }

    utils::console::clean_screen();
    println!("No hay mas oraciones por repasar. :)");
    println!();

    Ok(())
}
