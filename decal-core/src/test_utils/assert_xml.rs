use xmltree::Element;

/// Asserts equality between two XML documents.
///
/// # Arguments
/// - `left`: The source for first XML document.
/// - `right`: The source for second XML document.
pub(crate) fn assert_xml<L, R>(left: L, right: R)
where
    L: AsRef<str>,
    R: AsRef<str>,
{
    let left = left.as_ref();
    let right = right.as_ref();
    let left_xml = Element::parse(left.as_bytes()).expect("parsing error in left");
    let right_xml = Element::parse(right.as_bytes()).expect("parsing error in right");
    assert_eq!(left_xml, right_xml, "{left} != {right}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assertion_passes_on_identical_xml() {
        assert_xml(r#"<path d="0" />"#, r#"<path d="0" />"#);
    }

    #[test]
    #[should_panic]
    fn assertion_fails_on_different_xml() {
        assert_xml(r#"<path d="1" />"#, r#"<path d="0" />"#);
    }
}
