use crate::{
    db::schemas::worte_gender::{NewWorteGenderSchema, WorteGenderSchema},
    impl_test_helpers_for_schema,
};

impl_test_helpers_for_schema!(
    schema = WorteGenderSchema,
    new = NewWorteGenderSchema,
    snapshot = WorteGenderSnapshot,
    fields = [ id: i32, gender: String, artikel: String ],
    placeholders = [ created_at, deleted_at ]
);
