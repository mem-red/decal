pub(crate) fn str_sink<F>(write_fn: F) -> String
where
    F: FnOnce(&mut String) -> std::fmt::Result,
{
    let mut out = String::new();
    write_fn(&mut out).unwrap();
    out
}
