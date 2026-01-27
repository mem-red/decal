/// Executes a write operation into a temporary string buffer and returns the
/// result.
///
/// # Arguments
/// - `write_fn`: Closure that writes output into the provided string.
///
/// # Returns
/// - Final string.
pub(crate) fn str_sink<F>(write_fn: F) -> String
where
    F: FnOnce(&mut String) -> std::fmt::Result,
{
    let mut out = String::new();
    write_fn(&mut out).unwrap();
    out
}
