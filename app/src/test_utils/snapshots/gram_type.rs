use crate::{
    db::schemas::gram_type::{GramTypeSchema, NewGramTypeSchema},
    impl_test_helpers_for_schema,
};

impl_test_helpers_for_schema!(
    schema = GramTypeSchema,
    new = NewGramTypeSchema,
    snapshot = GramTypeSnapshot,
    fields = [ id: i32, code: String, name: String ],
    placeholders = [ created_at, deleted_at ]
);
