use crate::{
    db::schemas::niveau_liste::{NewNiveauListeSchema, NiveauListeSchema},
    impl_test_helpers_for_schema,
};

impl_test_helpers_for_schema!(
    schema = NiveauListeSchema,
    new = NewNiveauListeSchema,
    snapshot = NiveauListeSnapshot,
    fields = [ id: i32, niveau: String ],
    placeholders = [ created_at, deleted_at ]
);
