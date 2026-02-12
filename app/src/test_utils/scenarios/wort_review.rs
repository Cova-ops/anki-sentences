use crate::{
    db::schemas::wort_review::{EnumReviewDirection, InputWortReview},
    test_utils::scenarios::Scenario,
};

pub fn scenario_wort_review() -> Scenario<InputWortReview> {
    Scenario {
        initial: vec![
            InputWortReview {
                wort_id: 1,
                direction: EnumReviewDirection::ES2DE,
                interval: 1,
                ease_factor: 2.5,
                repetitions: 999,

                last_review: string_2_datetime("2025-01-10 12:00:00"),
                next_review: string_2_datetime("2025-01-20 12:00:00"),
            },
            InputWortReview {
                wort_id: 1,
                direction: EnumReviewDirection::DE2ES,
                interval: 12,
                ease_factor: 3.5,
                repetitions: 1,
                last_review: string_2_datetime("2025-01-10 12:00:00"),
                next_review: string_2_datetime("2025-01-20 12:00:00"),
            },
            InputWortReview {
                wort_id: 2,
                direction: EnumReviewDirection::ES2DE,
                interval: 1,
                ease_factor: 2.5,
                repetitions: 999,
                last_review: string_2_datetime("2025-01-10 12:00:00"),
                next_review: string_2_datetime("2025-01-20 12:00:00"),
            },
        ],
        update: vec![
            InputWortReview {
                wort_id: 1,
                direction: EnumReviewDirection::ES2DE,
                interval: 10,
                ease_factor: 1.3,
                repetitions: 1,
                last_review: string_2_datetime("2026-12-10 12:00:00"),
                next_review: string_2_datetime("2020-12-20 12:00:00"),
            },
            InputWortReview {
                wort_id: 1,
                direction: EnumReviewDirection::DE2ES,
                interval: 99,
                ease_factor: 99.9,
                repetitions: 9,
                last_review: string_2_datetime("2027-01-10 12:00:00"),
                next_review: string_2_datetime("2009-01-20 12:00:00"),
            },
            InputWortReview {
                wort_id: 2,
                direction: EnumReviewDirection::ES2DE,
                interval: 2,
                ease_factor: 2.5,
                repetitions: 333,
                last_review: string_2_datetime("1999-01-10 12:00:00"),
                next_review: string_2_datetime("2099-01-20 12:00:00"),
            },
        ],
    }
}
