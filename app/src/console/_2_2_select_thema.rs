use std::collections::{HashMap, HashSet};

use chrono::Utc;
use color_eyre::eyre::Result;
use inquire::MultiSelect;
use rand::seq::SliceRandom;
use rusqlite::Connection;

use crate::{
    db::{
        schemas::{
            setze_audio::SetzeAudioSchema,
            setze_review::{NewSetzeReviewSchema, SetzeReviewSchema},
        },
        setze::SetzeRepo,
        setze_audio::SetzeAudioRepo,
        setze_review::SetzeReviewRepo,
    },
    helpers::{console, review_state::ReviewState, time},
    utils,
};

pub fn menu_2_2_select_thema(conn: &mut Connection) -> Result<()> {
    let titles: Vec<String> = SetzeRepo::fetch_all_themas(conn)?;
    let offset: usize = 15;

    let ans = loop {
        let a = MultiSelect::new("Selecciona los temas a repasar:", titles.clone())
            .with_page_size(20)
            .prompt()
            .unwrap();
        if !a.is_empty() {
            break a;
        }
    };

    let mut ids_setze = SetzeRepo::fetch_id_where_thema(conn, &ans)?;

    let ids_audios: Vec<SetzeAudioSchema> = SetzeAudioRepo::fetch_by_id(conn, &ids_setze)?;
    let mut hash_audios: HashSet<i32> = HashSet::new();

    for id in ids_audios {
        hash_audios.insert(id.satz_id);
    }

    let mut seed_rand = rand::rng();
    ids_setze.shuffle(&mut seed_rand);

    let r = console::make_setze_exercise_repeat(conn, ids_setze, hash_audios, offset)?;

    // Obtenemos el id de las oraciones que respondio
    let setze_ids: Vec<i32> = r.1.iter().map(|(id, _)| *id).collect();

    // Obtenemos si estas oraciones ya tenian informacion hsitorica de revisiones anteriores
    let vec_setze_review = SetzeReviewRepo::fetch_by_satz_id(conn, &setze_ids)?;

    let mut hash_setze_review: HashMap<i32, SetzeReviewSchema> = HashMap::new();
    for sr in vec_setze_review {
        hash_setze_review.insert(sr.satz_id, sr);
    }

    let mut vec_new_setze_review: Vec<NewSetzeReviewSchema> = vec![];
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
        vec_new_setze_review.push(NewSetzeReviewSchema {
            satz_id,
            interval: review_state.interval,
            ease_factor: review_state.ease_factor,
            repetitions: review_state.repetitions,
            last_review: time::datetime_2_string(now),
            next_review: time::datetime_2_string(next),
        })
    }

    // guardamos en db la info de las revisiones
    SetzeReviewRepo::bulk_insert(conn, &vec_new_setze_review)?;

    if r.0 == 1 {
        return Ok(());
    }

    utils::clean_screen();
    println!("Oraciones finalizadas del tema.");
    println!();

    Ok(())
}
