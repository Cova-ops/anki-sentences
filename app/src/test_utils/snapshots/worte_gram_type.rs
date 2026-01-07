use crate::{
    db::schemas::worte_gram_type::{NewWorteGramTypeSchema, WorteGramTypeSchema},
    impl_test_helpers_for_schema,
};

impl_test_helpers_for_schema!(
    schema = WorteGramTypeSchema,
    new = NewWorteGramTypeSchema,
    snapshot = WorteGramTypeSnapshot,
    fields = [ id_worte: i32, id_gram_type: i32 ],
    placeholders = [ created_at, deleted_at ]
);
