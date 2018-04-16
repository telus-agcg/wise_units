#[macro_use]
extern crate wise_units;

#[test]
fn stuff() {
    use wise_units::test_atom::TestAtom;
    use wise_units::Classification;
    use wise_units::Measurable;

    let t = TestAtom::Thing;
    assert_eq!(t.classification(), Classification::Chemical);
    assert_eq!(t.definition().scalar(), 2.0);
    assert_eq!(t.lookup("[thing]").unwrap(), TestAtom::Thing);
}
