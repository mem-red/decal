use walkdir::WalkDir;

#[test]
fn ui() {
    let t = trybuild::TestCases::new();

    for entry in WalkDir::new("tests/ui/pass")
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            t.pass(path.to_str().unwrap());
        }
    }

    for entry in WalkDir::new("tests/ui/fail")
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            t.compile_fail(path.to_str().unwrap());
        }
    }
}
