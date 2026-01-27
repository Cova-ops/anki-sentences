use crate::{
    db::schemas::worte_review::{NewWorteReviewSchema, WorteReviewSchema},
    helpers::time::string_2_datetime,
    test_utils::traits::{AssertEqFields, SnapshotFields},
};

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub struct WorteReviewSnapshot {
    pub id: i32,
    pub wort_id: i32,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: String,
    pub next_review: String,

    // placeholders
    pub created_at: String,
    pub deleted_at: String,
}

impl SnapshotFields for WorteReviewSchema {
    type Output = WorteReviewSnapshot;

    fn snapshot(self) -> WorteReviewSnapshot {
        WorteReviewSnapshot {
            id: self.id,

            wort_id: self.wort_id,
            interval: self.interval,
            ease_factor: self.ease_factor,
            repetitions: self.repetitions,

            last_review: self.last_review.to_string(),
            next_review: self.next_review.to_string(),

            created_at: "<created_at>".into(),
            deleted_at: "<deleted_at>".into(),
        }
    }

    fn snapshot_ref(&self) -> WorteReviewSnapshot {
        WorteReviewSnapshot {
            id: self.id,

            wort_id: self.wort_id,
            interval: self.interval,
            ease_factor: self.ease_factor,
            repetitions: self.repetitions,

            last_review: self.last_review.to_string(),
            next_review: self.next_review.to_string(),

            created_at: "<created_at>".into(),
            deleted_at: "<deleted_at>".into(),
        }
    }
}

impl SnapshotFields for Vec<WorteReviewSchema> {
    type Output = Vec<WorteReviewSnapshot>;

    fn snapshot(self) -> Vec<WorteReviewSnapshot> {
        self.into_iter().map(|w| w.snapshot()).collect()
    }
    fn snapshot_ref(&self) -> Self::Output {
        self.iter().map(|w| w.snapshot_ref()).collect()
    }
}

// Schema vs New (1 a 1)
impl AssertEqFields<NewWorteReviewSchema> for WorteReviewSchema {
    fn assert_eq_fields(&self, expected: &NewWorteReviewSchema) {
        assert_eq!(self.wort_id, expected.wort_id);
        assert_eq!(self.interval, expected.interval);
        assert_eq!(self.ease_factor, expected.ease_factor);
        assert_eq!(self.repetitions, expected.repetitions);

        assert_eq!(
            self.last_review,
            string_2_datetime(Some(expected.last_review.as_str())).unwrap()
        );
        assert_eq!(
            self.next_review,
            string_2_datetime(Some(expected.next_review.as_str())).unwrap()
        );
    }
}

// Vec<Schema> vs Vec<New> (zip)
impl AssertEqFields<Vec<NewWorteReviewSchema>> for Vec<WorteReviewSchema> {
    fn assert_eq_fields(&self, expected: &Vec<NewWorteReviewSchema>) {
        assert_eq!(self.len(), expected.len(), "Length mismatch");

        for (a, e) in self.iter().zip(expected.iter()) {
            a.assert_eq_fields(e);
        }
    }
}
