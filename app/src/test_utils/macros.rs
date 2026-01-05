#[cfg(test)]
#[macro_export]
macro_rules! impl_test_helpers_for_schema {
    (
        schema = $Schema:ty,
        new = $New:ty,
        snapshot = $Snapshot:ident,
        fields = [ $( $field:ident : $fty:ty ),* $(,)? ],
        placeholders = [ $( $ph:ident ),* $(,)? ]
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[allow(dead_code)]
        pub struct $Snapshot {
            $( pub $field: $fty, )*
            $( pub $ph: String, )*
        }

        impl crate::test_utils::traits::SnapshotFields for $Schema {
            type Output = $Snapshot;

            fn snapshot_ref(&self) -> $Snapshot {
                $Snapshot {
                    $( $field: self.$field.clone(), )*
                    $( $ph: concat!("<", stringify!($ph), ">").to_string(), )*
                }
            }
            fn snapshot(self) -> $Snapshot {
                $Snapshot {
                    $( $field: self.$field, )*
                    $( $ph: concat!("<", stringify!($ph), ">").to_string(), )*
                }
            }
        }

        impl crate::test_utils::traits::SnapshotFields for Vec<$Schema> {
            type Output = Vec<$Snapshot>;

            fn snapshot(self) -> Vec<$Snapshot> {
                self.into_iter().map(|s| s.snapshot()).collect()
            }
            fn snapshot_ref(&self) -> Vec<$Snapshot> {
                self.iter().map(|s| s.snapshot_ref()).collect()
            }
        }

        impl crate::test_utils::traits::SnapshotFields for Option<$Schema> {
            type Output = Option<$Snapshot>;

            fn snapshot_ref(&self) -> Option<$Snapshot> {
                self.as_ref().map(|v| v.snapshot_ref())
            }
            fn snapshot(self) -> Option<$Snapshot> {
                self.map(|v| v.snapshot())
            }
        }

        // Schema vs New (1 a 1)
        impl crate::test_utils::traits::AssertEqFields<$New> for $Schema {
            fn assert_eq_fields(&self, expected: &$New) {
                $(
                    assert_eq!(self.$field, expected.$field);
                )*
            }
        }

        // Vec<Schema> vs Vec<New> (zip)
        impl crate::test_utils::traits::AssertEqFields<Vec<$New>> for Vec<$Schema> {
            fn assert_eq_fields(&self, expected: &Vec<$New>) {
                assert_eq!(self.len(), expected.len(), "Length mismatch");

                for (a, e) in self.iter().zip(expected.iter()) {
                    a.assert_eq_fields(e);
                }
            }
        }
    };
}
