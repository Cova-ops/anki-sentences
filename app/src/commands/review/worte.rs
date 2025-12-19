use std::collections::{HashMap, HashSet};

use crate::{
    console::cli::ReviewWorteSection,
    db::{
        get_conn,
        schemas::worte_review::{NewWorteReviewSchema, WorteReviewSchema},
        worte_audio::WorteAudioRepo,
        worte_review::WorteReviewRepo,
    },
    helpers::{self, review_state::ReviewState, time, toml::AppConfig},
    utils,
};

use chrono::Utc;
use color_eyre::Result;
use rand::seq::SliceRandom;

pub fn run(
    config: &AppConfig,
    section: ReviewWorteSection,
    batch: usize,
    no_shuffle: bool,
) -> Result<()> {
    let mut conn = get_conn(config.get_database_path()?)?;

    let mut ids_worte: Vec<i32> = match section {
        ReviewWorteSection::NewAndReview => {
            let date_review = time::today_local_string(1);
            WorteReviewRepo::fetch_review_wort_id_by_day(&conn, date_review)?
        }
        _ => todo!("Aguantame papito"),
    };

    let ids_audios = if config.is_audio_enable()? {
        WorteAudioRepo::fetch_by_id(&conn, &ids_worte)?
    } else {
        Vec::new()
    };
    let hash_audios: HashSet<i32> = ids_audios.iter().map(|ia| ia.wort_id).collect();

    if !no_shuffle {
        let mut rng = rand::rng();
        ids_worte.shuffle(&mut rng);
    }

    let manage_audio = helpers::audios::ManageAudios::new(
        config.get_path_audios_worte()?,
        config.get_path_audios_setze()?,
    );
    let r = helpers::console::make_worte_exercise_repeat(
        &conn,
        ids_worte,
        hash_audios,
        &manage_audio,
        batch,
    )?;

    // Obtenemos el id de las palabras que respondio
    let wort_ids: Vec<i32> = r.1.iter().map(|(id, _)| *id).collect();

    // Obtenemos si estas palabras ya tenian informacion hsitorica de revisiones anteriores
    let vec_worte_review = WorteReviewRepo::fetch_by_wort_id(&conn, &wort_ids)?;

    let hash_worte_review: HashMap<i32, WorteReviewSchema> = vec_worte_review
        .into_iter()
        .map(|wr| (wr.wort_id, wr))
        .collect();

    let mut vec_new_worte_review: Vec<NewWorteReviewSchema> = vec![];
    let now = Utc::now();

    // Recorremos el arreglo de palabras que respondio el usuario
    for wort in r.1 {
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
        vec_new_worte_review.push(NewWorteReviewSchema {
            wort_id,
            interval: review_state.interval,
            ease_factor: review_state.ease_factor,
            repetitions: review_state.repetitions,
            last_review: helpers::time::datetime_2_string(now),
            next_review: helpers::time::datetime_2_string(next),
        })
    }

    // guardamos en db la info de las revisiones
    WorteReviewRepo::bulk_insert(&mut conn, &vec_new_worte_review)?;

    if r.0 == 1 {
        return Ok(());
    }

    utils::clean_screen();
    println!("No hay mas palabras por estudiar. :)");
    println!();

    Ok(())
}
