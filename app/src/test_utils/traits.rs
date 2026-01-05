pub trait SnapshotFields {
    type Output;
    fn snapshot(self) -> Self::Output;
}

pub trait AssertEqFields<Expected> {
    fn assert_eq_fields(&self, expected: &Expected);
}
