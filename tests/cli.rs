#[macro_use] extern crate assert_cli;

const LINK_CHECKER: &'static str = "target/debug/anchor-link-checker";

#[test]
fn pass() {
    assert_cli!(
        LINK_CHECKER, &["tests/fixtures/pass.html"] =>
        Success, "Yay!"
    ).unwrap();
}

#[test]
fn missing_file() {
    assert_cli!(
        LINK_CHECKER, &["tests/fixtures/missing_file.html"] =>
        Error 1, "Couldn't read file"
    ).unwrap();
}

#[test]
fn fail() {
    assert_cli!(
        LINK_CHECKER, &["tests/fixtures/fail.html"] =>
        Error 1, "Missing links: {\n    \"missing-link\"\n}"
    ).unwrap();
}
