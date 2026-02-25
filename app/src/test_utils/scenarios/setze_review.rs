use crate::{
    db::schemas::setze_review::InputSetzeReview, helpers::time::string_2_datetime,
    test_utils::scenarios::Scenario,
};

pub fn scenario_setze_review() -> Scenario<InputSetzeReview> {
    Scenario {
        initial: vec![
            InputSetzeReview {
                satz_id: 1,
                interval: 5,
                ease_factor: 1.3,
                repetitions: 10,
                last_review: string_2_datetime("2016-01-02 20:00:00").unwrap(),
                next_review: string_2_datetime("2017-01-02 20:00:00").unwrap(),
            },
            InputSetzeReview {
                satz_id: 2,
                interval: 1,
                ease_factor: 2.3,
                repetitions: 20,
                last_review: string_2_datetime("2016-12-02 20:00:00").unwrap(),
                next_review: string_2_datetime("2017-12-02 20:00:00").unwrap(),
            },
        ],
        update: vec![
            InputSetzeReview {
                satz_id: 1,
                interval: 2,
                ease_factor: 1.4,
                repetitions: 100,
                last_review: string_2_datetime("2036-01-02 20:00:00").unwrap(),
                next_review: string_2_datetime("2037-01-02 20:00:00").unwrap(),
            },
            InputSetzeReview {
                satz_id: 2,
                interval: 3,
                ease_factor: 2.0,
                repetitions: 200,
                last_review: string_2_datetime("2036-12-02 20:00:00").unwrap(),
                next_review: string_2_datetime("2037-12-02 20:00:00").unwrap(),
            },
        ],
        update_id: vec![],
    }
}
