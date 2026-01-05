// app/src/test_utils/macros.rs
// Solo compila en tests
#[cfg(test)]
#[macro_export]
macro_rules! impl_test_helpers_for_schema {
    (
        schema = $Schema:ty,
        snapshot = $Snapshot:ident,
        fields = [ $( $field:ident : $fty:ty ),* $(,)? ],
        placeholders = [ $( $ph:ident ),* $(,)? ]
    ) => {
        // --- Snapshot struct ---
        #[derive(Debug)]
        #[allow(dead_code)]
        pub struct $Snapshot {
            $( pub $field: $fty, )*
            $( pub $ph: String, )*
        }

        // --- Snapshot trait ---
        pub trait SnapshotFields {
            type Output;
            fn snapshot(self) -> Self::Output;
        }

        impl SnapshotFields for $Schema {
            type Output = $Snapshot;

            fn snapshot(self) -> $Snapshot {
                $Snapshot {
                    $( $field: self.$field, )*
                    $( $ph: concat!("<", stringify!($ph), ">").to_string(), )*
                }
            }
        }

        impl SnapshotFields for Vec<$Schema> {
            type Output = Vec<$Snapshot>;

            fn snapshot(self) -> Vec<$Snapshot> {
                self.into_iter().map(|s| s.snapshot()).collect()
            }
        }

        // --- Assert helper ---
        pub trait AssertFields {
            fn assert_fields(&self, $( $field: $fty ),* );
        }

        impl AssertFields for $Schema {
            fn assert_fields(&self, $( $field: $fty ),* ) {
                $(
                    assert_eq!(self.$field, $field);
                )*
            }
        }
    };
}
