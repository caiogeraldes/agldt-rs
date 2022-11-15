#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-parse.rs");
    t.compile_fail("tests/02-conflicting-feature-postag.rs");
    t.pass("tests/03-postag-attribute.rs");
    t.pass("tests/04-postagindex-attribute.rs");
}
