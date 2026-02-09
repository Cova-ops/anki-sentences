pub trait AssertEqFields<Expected> {
    fn assert_eq_fields(&self, expected: &Expected);
}
