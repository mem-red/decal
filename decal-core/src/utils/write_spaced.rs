use std::fmt::Write;

/// Writes elements from an iterator separated by single spaces.
///
/// # Arguments
/// - `out`: The output sink receiving the content.
/// - `itr`: The element iterator.
/// - `write_element`: The closure called with each element to write them to the
///   output sink.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test_utils::str_sink,
        utils::FloatWriter,
    };
    use std::iter;

    #[test]
    fn writes_nothing_for_empty_iterator() {
        assert_eq!(
            str_sink(
                |out| write_spaced(out, iter::empty::<f32>(), |out, value| out
                    .write_float(value))
            ),
            ""
        );
    }

    #[test]
    fn writes_single_element_without_spaces() {
        assert_eq!(
            str_sink(|out| write_spaced(out, iter::once(15), |out, value| write!(out, "{value}"))),
            "15"
        );
    }

    #[test]
    fn writes_multiple_elements_with_spaces() {
        assert_eq!(
            str_sink(
                |out| write_spaced(out, [1, 2, 3, 4, 5].into_iter(), |out, value| write!(
                    out,
                    "{value}"
                ))
            ),
            "1 2 3 4 5"
        );
    }
}
