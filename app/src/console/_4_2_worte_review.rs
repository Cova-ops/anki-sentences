use std::collections::{HashMap, HashSet};

use chrono::Utc;
use color_eyre::eyre::Result;
use rand::seq::SliceRandom;
use rusqlite::Connection;

use crate::{
    db::{
        schemas::{
            worte_audio::WorteAudioSchema,
            worte_review::{NewWorteReviewSchema, WorteReviewSchema},
        },
        worte_audio::WorteAudioRepo,
        worte_review::WorteReviewRepo,
    },
    helpers::{self, review_state::ReviewState, time},
    utils,
};

pub fn menu_4_2_worte_review(conn: &mut Connection) -> Result<()> {
    let offset: usize = 20;

    // 1) Obtenemos ids de las palabras a repasar
    let date_review = time::today_local_string(1);
    let mut ids_worte: Vec<i32> = WorteReviewRepo::fetch_review_wort_id_by_day(conn, date_review)?;

    let ids_audios: Vec<WorteAudioSchema> = WorteAudioRepo::fetch_by_id(conn, &ids_worte)?;
    let mut hash_audios: HashSet<i32> = HashSet::new();

    for id in ids_audios {
        hash_audios.insert(id.wort_id);
    }

    // Les hacemos un shuffle
    let mut seed_rand = rand::rng();
    ids_worte.shuffle(&mut seed_rand);

    // le hacemos el ejercicio al usuario
    let r = helpers::console::make_worte_exercise_repeat(conn, ids_worte, hash_audios, offset)?;

    // Obtenemos el id de las palabras que respondio
    let wort_ids: Vec<i32> = r.1.iter().map(|(id, _)| *id).collect();

    // Obtenemos si estas palabras ya tenian informacion hsitorica de revisiones anteriores
    let vec_worte_review = WorteReviewRepo::fetch_by_wort_id(conn, &wort_ids)?;

    let mut hash_worte_review: HashMap<i32, WorteReviewSchema> = HashMap::new();
    for wr in vec_worte_review {
        hash_worte_review.insert(wr.wort_id, wr);
    }

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
            last_review: time::datetime_2_string(now),
            next_review: time::datetime_2_string(next),
        })
    }

    // guardamos en db la info de las revisiones
    WorteReviewRepo::bulk_insert(conn, &vec_new_worte_review)?;

    if r.0 == 1 {
        return Ok(());
    }

    utils::clean_screen();
    println!("No hay mas palabras por estudiar. :)");
    println!();

    Ok(())
}
