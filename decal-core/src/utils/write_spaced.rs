use std::fmt::Write;

pub(crate) fn write_spaced<W, I, F, T>(
    out: &mut W,
    mut itr: I,
    mut write_element: F,
) -> std::fmt::Result
where
    W: Write,
    I: Iterator<Item = T>,
    F: FnMut(&mut W, T) -> std::fmt::Result,
{
    if let Some(first) = itr.next() {
        write_element(out, first)?;

        for value in itr {
            out.write_char(' ')?;
            write_element(out, value)?;
        }
    }

    Ok(())
}
