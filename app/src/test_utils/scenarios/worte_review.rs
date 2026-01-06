use crate::{
    db::schemas::worte_review::{NewWorteReviewSchema, ReviewDirection},
    test_utils::scenarios::Scenario,
};

pub fn scenario_worte_review() -> Scenario<NewWorteReviewSchema> {
    Scenario {
        initial: vec![
            NewWorteReviewSchema {
                wort_id: 1,
                direction: ReviewDirection::ES2DE.to_string(),
                interval: 1,
                ease_factor: 2.5,
                repetitions: 999,
                last_review: "2025-01-10 12:00:00".into(),
                next_review: "2025-01-20 12:00:00".into(),
            },
            NewWorteReviewSchema {
                wort_id: 1,
                direction: ReviewDirection::DE2ES.to_string(),
                interval: 12,
                ease_factor: 3.5,
                repetitions: 1,
                last_review: "2025-01-10 12:00:00".into(),
                next_review: "2025-01-20 12:00:00".into(),
            },
            NewWorteReviewSchema {
                wort_id: 2,
                direction: ReviewDirection::ES2DE.to_string(),
                interval: 1,
                ease_factor: 2.5,
                repetitions: 999,
                last_review: "2025-01-10 12:00:00".into(),
                next_review: "2025-01-20 12:00:00".into(),
            },
        ],
        update: vec![
            NewWorteReviewSchema {
                wort_id: 1,
                direction: ReviewDirection::ES2DE.to_string(),
                interval: 10,
                ease_factor: 1.3,
                repetitions: 1,
                last_review: "2026-12-10 12:00:00".into(),
                next_review: "2020-12-20 12:00:00".into(),
            },
            NewWorteReviewSchema {
                wort_id: 1,
                direction: ReviewDirection::DE2ES.to_string(),
                interval: 99,
                ease_factor: 99.9,
                repetitions: 9,
                last_review: "2027-01-10 12:00:00".into(),
                next_review: "2009-01-20 12:00:00".into(),
            },
            NewWorteReviewSchema {
                wort_id: 2,
                direction: ReviewDirection::ES2DE.to_string(),
                interval: 2,
                ease_factor: 2.5,
                repetitions: 333,
                last_review: "1999-01-10 12:00:00".into(),
                next_review: "2099-01-20 12:00:00".into(),
            },
        ],
    }
}
