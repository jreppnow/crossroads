#[test]
fn good() {
    let t = trybuild::TestCases::new();
    t.pass("tests/good/*.rs");
}
